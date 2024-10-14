struct Months {
    months: Vec<String>,
    current: usize,
}

impl Months {
    pub fn winter() -> Months {
        Months {
            months: vec![
                "December".to_string(),
                "January".to_string(),
                "February".to_string(),
            ],
            current: 0,
        }
    }

    pub fn new() -> Months {
        Months {
            months: vec![
                "January".to_string(),
                "February".to_string(),
                "March".to_string(),
                "April".to_string(),
                "May".to_string(),
                "June".to_string(),
                "July".to_string(),
                "August".to_string(),
                "September".to_string(),
                "October".to_string(),
                "November".to_string(),
                "December".to_string(),
            ],
            current: 0,
        }
    }
}

impl Iterator for Months {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.months.len() {
            let x = Some(self.months[self.current].clone());
            self.current += 1;
            x
        } else {
            None
        }
    }
}

#[test]
fn test1() {
    let mm = Months::new();
    for x in mm {
        println!("{}", x);
    }
}

#[test]
fn test2() {
    let mm = Months::new();
    mm.take(1)
        .for_each(|x| println!("{}", x));
}

#[test]
fn test3() {
    let mm = Months::winter();

    for m in mm {
        println!("{}", m);
    }

    // for m in mm {
    //     println!("{}", m);
    // }
}

#[test]
fn test4() {
    let mm = Months::winter();
    mm.for_each(|x| println!("{}", x));
}
