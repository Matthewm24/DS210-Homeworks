use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ColumnVal {
    One(String),
    Two(bool),
    Three(f64),
    Four(i64),
}

#[derive(Debug)]
struct DataFrame {
    columns: HashMap<String, Vec<ColumnVal>>,
    labels: Vec<String>,
}

impl DataFrame {
    fn new() -> Self {
        DataFrame {
            columns: HashMap::new(),
            labels: Vec::new(),
        }
    }

    fn read_csv(&mut self, path: &str, types: &Vec<u32>) {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(true)
            .flexible(true)
            .from_path(path)
            .unwrap();

        let headers = rdr.headers().unwrap();
        self.labels = headers.iter().map(|s| s.to_string()).collect();

        for result in rdr.records() {
            let r = result.unwrap();

            for (i, elem) in r.iter().enumerate() {
                let label: &String = &self.labels[i];
                let val = match types[i] {
                    1 => ColumnVal::One(elem.to_string()),
                    2 => {
                        let bool_val = match elem.trim().to_lowercase().as_str() {
                            "true" => true,
                            "false" => false,
                            _ => false,
                        };
                        ColumnVal::Two(bool_val)
                    }
                    3 => ColumnVal::Three(elem.parse::<f64>().unwrap()),
                    4 => ColumnVal::Four(elem.parse::<i64>().unwrap()),
                    _ => ColumnVal::One(elem.to_string()),
                };

                self.columns
                    .entry(label.clone())
                    .or_insert_with(Vec::new)
                    .push(val);
            }
        }
    }

    fn print(&self) {
        let mut col_widths: Vec<usize> = self.labels.iter().map(|label| label.len()).collect();

        for (i, label) in self.labels.iter().enumerate() {
            if let Some(col) = self.columns.get(label) {
                for val in col {
                    let val_len = match val {
                        ColumnVal::One(s) => s.len(),
                        ColumnVal::Two(b) => b.to_string().len(),
                        ColumnVal::Three(f) => format!("{:.1}", f).len(),
                        ColumnVal::Four(i) => i.to_string().len(),
                    };
                    col_widths[i] = col_widths[i].max(val_len);
                }
            }
        }

        for (i, label) in self.labels.iter().enumerate() {
            print!("{:<width$} ", label, width = col_widths[i]);
        }
        println!();

        if let Some(first_col) = self.columns.get(&self.labels[0]) {
            for i in 0..first_col.len() {
                for (j, label) in self.labels.iter().enumerate() {
                    if let Some(col) = self.columns.get(label) {
                        match &col[i] {
                            ColumnVal::One(s) => print!("{:<width$} ", s, width = col_widths[j]),
                            ColumnVal::Two(b) => print!("{:<width$} ", b, width = col_widths[j]),
                            ColumnVal::Three(f) => {
                                print!("{:<width$.1} ", f, width = col_widths[j])
                            }
                            ColumnVal::Four(i) => print!("{:<width$} ", i, width = col_widths[j]),
                        }
                    }
                }
                println!();
            }
        }
    }

    fn add_column(&mut self, label: String, data: Vec<ColumnVal>) -> DataFrame {
        let mut new_df = DataFrame::new();
        new_df.labels = self.labels.clone();
        new_df.columns = self.columns.clone();

        new_df.labels.push(label.clone());
        new_df.columns.insert(label, data);

        new_df
    }

    fn merge_frame(&self, other: &DataFrame) -> DataFrame {
        let mut new_df = DataFrame::new();
        new_df.labels = self.labels.clone();

        for label in &self.labels {
            let mut combined = self.columns[label].clone();
            combined.extend(other.columns[label].clone());
            new_df.columns.insert(label.clone(), combined);
        }
        new_df
    }

    fn find_columns(&self, labels: &[String]) -> DataFrame {
        let mut new_df = DataFrame::new();
        for label in labels {
            new_df.labels.push(label.clone());
            new_df
                .columns
                .insert(label.clone(), self.columns[label].clone());
        }
        new_df
    }

    fn restrict_columns(&self, labels: &[String]) -> DataFrame {
        let mut new_df = DataFrame::new();
        new_df.labels = labels.to_vec();

        for label in labels {
            new_df
                .columns
                .insert(label.clone(), self.columns[label].clone());
        }
        new_df
    }

    fn filter(&self, label: &str, operation: fn(&ColumnVal) -> bool) -> DataFrame {
        let mut new_df = DataFrame::new();
        new_df.labels = self.labels.clone();

        let indices: Vec<usize> = self.columns[label]
            .iter()
            .enumerate()
            .filter(|(_, val)| operation(val))
            .map(|(i, _)| i)
            .collect();

        for label in &self.labels {
            let filtered_data: Vec<ColumnVal> = indices
                .iter()
                .map(|&i| self.columns[label][i].clone())
                .collect();
            new_df.columns.insert(label.clone(), filtered_data);
        }

        new_df
    }

    fn column_op(
        &self,
        labels: &[String],
        op: fn(&[Vec<ColumnVal>]) -> Vec<ColumnVal>,
    ) -> Vec<ColumnVal> {
        let columns: Vec<Vec<ColumnVal>> = labels
            .iter()
            .map(|label| self.columns[label].clone())
            .collect();
        op(&columns)
    }

    fn median(&self, label: &str) -> f64 {
        let result = self.column_op(&[label.to_string()], |columns| {
            let mut values: Vec<f64> = columns[0]
                .iter()
                .filter_map(|val| {
                    if let ColumnVal::Three(f) = val {
                        Some(*f)
                    } else {
                        None
                    }
                })
                .collect();

            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let len = values.len();
            let median = if len % 2 == 0 {
                (values[len / 2 - 1] + values[len / 2]) / 2.0
            } else {
                values[len / 2]
            };
            vec![ColumnVal::Three(median)]
        });

        if let ColumnVal::Three(median) = result[0] {
            median
        } else {
            0.0
        }
    }

    fn sub_columns(&self, label1: &str, label2: &str) -> Vec<i64> {
        let result = self.column_op(&[label1.to_string(), label2.to_string()], |columns| {
            let mut differences = Vec::new();
            for i in 0..columns[0].len() {
                if let (ColumnVal::Four(val1), ColumnVal::Four(val2)) =
                    (&columns[0][i], &columns[1][i])
                {
                    differences.push(ColumnVal::Four(val1 - val2));
                }
            }
            differences
        });

        result
            .into_iter()
            .filter_map(|val| {
                if let ColumnVal::Four(i) = val {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() {
    let types = vec![1, 4, 3, 4, 4, 2];

    let mut df1 = DataFrame::new();
    df1.read_csv("data.csv", &types);

    println!("\nOriginal DataFrame:");
    df1.print();

    let new_data = vec![
        ColumnVal::Two(false), // Kareem
        ColumnVal::Two(false), // Karl
        ColumnVal::Two(true),  // LeBron
        ColumnVal::Two(false), // Kobe
        ColumnVal::Two(false), // Michael
    ];
    let df2 = df1.add_column("IsAllStar".to_string(), new_data);
    println!("\nDataFrame with new column:");
    df2.print();

    let mut df3 = DataFrame::new();
    df3.read_csv("data.csv", &types);
    let merged_df = df1.merge_frame(&df3);
    println!("\nMerged DataFrame:");
    merged_df.print();

    let selected_columns = df1.find_columns(&["Name".to_string(), "PPG".to_string()]);
    println!("\nSelected columns:");
    selected_columns.print();

    let restricted_df = df1.restrict_columns(&["Name".to_string(), "TotalPoints".to_string()]);
    println!("\nRestricted columns:");
    restricted_df.print();

    let high_scorers = df1.filter("PPG", |val| {
        if let ColumnVal::Three(ppg) = val {
            *ppg > 20.0
        } else {
            false
        }
    });
    println!("\nPlayers with PPG > 20:");
    high_scorers.print();

    let median_ppg = df1.median("PPG");
    println!("\nMedian PPG: {:.2}", median_ppg);

    let differences = df1.sub_columns("TotalPoints", "YearBorn");
    println!("\nTotalPoints - YearBorn differences:");
    for (i, diff) in differences.iter().enumerate() {
        if let ColumnVal::One(name) = &df1.columns["Name"][i] {
            println!("{}: {}", name, diff);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv() {
        let mut df = DataFrame::new();
        let types = vec![1, 4, 3, 4, 4, 2]; // Name, Number, PPG, YearBorn, TotalPoints, LikesPizza
        df.read_csv("src/data.csv", &types);

        assert_eq!(df.labels.len(), 6);
        assert!(df.labels.contains(&"Name".to_string()));
        assert!(df.labels.contains(&"PPG".to_string()));

        if let Some(name_col) = df.columns.get("Name") {
            assert!(matches!(name_col[0], ColumnVal::One(_)));
        }
        if let Some(ppg_col) = df.columns.get("PPG") {
            assert!(matches!(ppg_col[0], ColumnVal::Three(_)));
        }
    }

    #[test]
    fn test_filter_ppg() {
        let mut df = DataFrame::new();
        let types = vec![1, 4, 3, 4, 4, 2];
        df.read_csv("src/data.csv", &types);

        let filtered = df.filter("PPG", |val| {
            if let ColumnVal::Three(ppg) = val {
                *ppg > 20.0
            } else {
                false
            }
        });

        // Verify filtered results
        if let Some(ppg_col) = filtered.columns.get("PPG") {
            for val in ppg_col {
                if let ColumnVal::Three(ppg) = val {
                    assert!(*ppg > 20.0);
                }
            }
        }
    }

    #[test]
    fn test_median_calculation() {
        let mut df = DataFrame::new();
        let types = vec![1, 4, 3, 4, 4, 2];
        df.read_csv("src/data.csv", &types);

        let median_ppg = df.median("PPG");
        assert!(median_ppg > 0.0);

        assert!(median_ppg >= 0.0 && median_ppg <= 50.0);
    }
}
