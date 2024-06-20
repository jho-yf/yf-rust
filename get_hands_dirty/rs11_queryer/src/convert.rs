use anyhow::{anyhow, Result};
use polars::{lazy::dsl::Operator, prelude::*};
use sqlparser::ast::{
    BinaryOperator as SqlBinaryOperator, Expr as SqlExpr, Offset as SqlOffset, OrderByExpr, Select, SelectItem, SetExpr, Statement, TableFactor, TableWithJoins, Value as SqlValue};

/// 解析的 SQL Statement 数据结构
pub struct Sql<'a> {
    pub(crate) selection: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: &'a str,
    pub(crate) order_by: Vec<(String, bool)>,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<usize>,
}

// 因为 Rust trait 的孤儿规则，如果想对已有的类型实现已有的 trait, 需要简单包装一下

pub struct Expression(pub(crate) Box<SqlExpr>);
pub struct Operation(pub(crate) SqlBinaryOperator);
pub struct Projection<'a>(pub(crate) &'a SelectItem);
pub struct Source<'a>(pub(crate) &'a [TableWithJoins]);
pub struct Order<'a>(pub(crate) &'a OrderByExpr);
pub struct Offset<'a>(pub(crate) &'a SqlOffset);
pub struct Limit<'a>(pub(crate) &'a SqlExpr);
pub struct Value(pub(crate) SqlValue);


/// 把 SqlParser 解析出来的 Statement 转换成自定义结构
impl <'a> TryFrom<&'a Statement> for Sql<'a> {
    type Error = anyhow::Error;
    fn try_from(sql: &'a Statement) -> Result<Self, Self::Error> {
        match sql {
            Statement::Query(q) => {
                let offset = q.offset.as_ref();
                let limit = q.limit.as_ref();
                let orders = &q.order_by;

                let Select {
                    from: table_with_joins,
                    selection: where_clause,
                    projection,

                    group_by: _,
                    ..
                } = match q.body.as_ref() {
                    SetExpr::Select(statement) => {
                        statement.as_ref()
                    },
                    _ => return Err(anyhow!("unsupported query type")), 
                };

                let source = Source(table_with_joins).try_into()?;
                let condition = match where_clause {
                    Some(expr) => Some(Expression(Box::new(expr.to_owned())).try_into()?),
                    None => None,
                };
                
                let mut selection = Vec::with_capacity(8);
                for p in projection {
                    let expr = Projection(p).try_into()?;
                    selection.push(expr);
                }

                let mut order_by = Vec::new();
                for expr in orders {
                    order_by.push(Order(expr).try_into()?);
                }

                let offset = offset.map(|v| Offset(v).into());
                let limit = limit.map(|v| Limit(v).into());

                Ok(Sql {
                    selection,
                    condition,
                    source,
                    order_by,
                    offset,
                    limit,
                })
            }
            _ => Err(anyhow!("unsupported statement type"))
        }        
    }
}


/// 把 sqlparser 的 Expression 转换为 DataFrame 的 Expr
impl TryFrom<Expression> for Expr {
    type Error = anyhow::Error;
    fn try_from(expr: Expression) -> Result<Self, Self::Error> {
        match *expr.0 {
            SqlExpr::Value(v) => Ok(Self::Literal(Value(v).try_into()?)),
            v => Err(anyhow!("expr {:#?} is not supported", v)),
        }
    }
}

impl TryFrom<Operation> for Operator {
    
    type Error = anyhow::Error;

    fn try_from(op: Operation) -> Result<Self, Self::Error> {
        match op.0 {
            SqlBinaryOperator::Plus => Ok(Self::Plus),
            v => Err(anyhow!("operation {:#?} is not supported", v)),
        }
    }

}


/// 把 sqlparser 的 SelectItem 转换为 Expr
impl<'a> TryFrom<Projection<'a>> for Expr {
    type Error = anyhow::Error;
    fn try_from(p: Projection<'a>) -> Result<Self, Self::Error> {
        match p.0 {
            SelectItem::UnnamedExpr(SqlExpr::Identifier(id)) => Ok(col(&id.to_string())),
            SelectItem::ExprWithAlias { 
                expr: SqlExpr::Identifier(id), 
                alias,
            } => Ok(Expr::Alias(
                Arc::new(Expr::Column(Arc::from(id.to_string()))),
                Arc::from(alias.to_string()),
            )),
            item => Err(anyhow!("projection {:#?} is not supported", item))
        }
    }
}

impl<'a> TryFrom<Source<'a>> for &'a str {
    type Error = anyhow::Error;
    fn try_from(source: Source<'a>) -> Result<Self, Self::Error> {
        if source.0.len() != 1 {
            return Err(anyhow!("only support single data source at the moment"));
        }

        let table = &source.0[0];
        if !table.joins.is_empty() {
            return Err(anyhow!("not support joint data source at the moment"));
        }

        match &table.relation {
            TableFactor::Table { name, .. } => Ok(&name.0.first().unwrap().value),
            _ => Err(anyhow!("only support table")),            
        }
    }
}

// 将 SqlParser 的 OrderByExpr 转换为 (String, bool)
impl<'a> TryFrom<Order<'a>> for (String, bool) {
    type Error = anyhow::Error;
    fn try_from(order: Order<'a>) -> Result<Self, Self::Error> {
        let name = match &order.0.expr {
            SqlExpr::Identifier(id) => id.to_string(),
            expr => {
                return Err(anyhow!("only support identifier for order by, got{}", expr));
            }
        };

        Ok((name, !order.0.asc.unwrap_or(true)))
    }
}

// 将 SqlParser 的 Offset expr 转换为 i64
impl<'a> From<Offset<'a>> for i64 {
    fn from(offset: Offset<'a>) -> Self {
        match offset.0 {
            SqlOffset {
                value: SqlExpr::Value(SqlValue::Number(v, _b)),
                ..
            } => v.parse().unwrap_or(0),
            _ => 0,
        }
    }
}

// 将 SqlParer 的 Limit expr 转换为 usize
impl<'a> From<Limit<'a>> for usize {
    fn from(l: Limit<'a>) -> Self {
        match l.0 {
            SqlExpr::Value(SqlValue::Number(v, _b)) => v.parse().unwrap_or(usize::MAX),
            _ => usize::MAX,
        }
    }
}

impl TryFrom<Value> for LiteralValue {
    type Error = anyhow::Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.0 {
            SqlValue::Number(v, _) => Ok(LiteralValue::Float64(v.parse().unwrap())),
            SqlValue::Boolean(v) => Ok(LiteralValue::Boolean(v)),
            SqlValue::Null => Ok(LiteralValue::Null),
            v => Err(anyhow!("value {} is not supported", v))
        }
    }
}