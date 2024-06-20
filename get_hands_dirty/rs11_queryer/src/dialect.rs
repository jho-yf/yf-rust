use sqlparser::dialect::Dialect;

#[derive(Debug, Default)]
pub struct YfDialect;

// 自定义SQL方言：支持从url读取数据
impl Dialect for YfDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) 
            || ('A'..='Z').contains(&ch) 
            || ('0'..='9').contains(&ch)
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::parser::Parser;

    #[test]
    fn it_works() {
        let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
        
        let sql = format!("SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
                FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5", 
                url
            );
        assert!(Parser::parse_sql(&YfDialect::default(), &sql).is_ok());
    }
}