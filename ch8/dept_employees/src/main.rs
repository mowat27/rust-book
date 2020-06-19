use std::collections::HashMap;

fn main() {
    let mut co = Company::new();
    co.add_str("add Dave to HR");
    co.add_str("add Betty to Engineering");
    co.add_str("add Sally to HR");
    println!("All: {}", co.report(Query::All));
    println!("HR: {}", co.report(Query::dept("HR")));
    println!("Engineering {}", co.report(Query::dept("Engineering")));
}

struct Company {
    payroll: HashMap<String, Vec<String>>,
}

enum Query {
    All,
    Dept(String),
}

impl Query {
    fn dept(dept: &str) -> Query {
        Query::Dept(String::from(dept))
    }
}

impl Company {
    fn new() -> Company {
        Company {
            payroll: HashMap::new(),
        }
    }

    fn add_str(&mut self, txt: &str) {
        let v: Vec<&str> = txt.split(' ').collect();
        let (emp, dept) = (String::from(v[1]), String::from(v[3]));
        let employees = self.payroll.entry(dept).or_insert(vec![]);
        employees.push(emp);
        employees.sort();
    }

    fn report(&self, q: Query) -> String {
        match q {
            Query::All => {
                let mut result: Vec<String> = self
                    .payroll
                    .values()
                    .flatten()
                    .map(|s| String::from(s))
                    .collect();
                result.sort();
                result.join(", ")
            }
            Query::Dept(dept) => match self.payroll.get(&dept) {
                Some(employees) => employees.join(", "),
                None => String::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let co = Company::new();
        assert_eq!("", co.report(Query::All));
    }

    #[test]
    fn sales() {
        let mut co = Company::new();
        co.add_str("add Sally to Sales");
        co.add_str("add Alice to Sales");
        co.add_str("add Bob to Sales");
        assert_eq!("Alice, Bob, Sally", co.report(Query::dept("Sales")));
    }

    #[test]
    fn company() {
        let mut co = Company::new();
        co.add_str("add Andy to Marketing");
        co.add_str("add Sally to Sales");
        co.add_str("add Alice to Sales");
        co.add_str("add Bob to Sales");
        assert_eq!("Alice, Andy, Bob, Sally", co.report(Query::All));
    }
}
