pub struct SelectionsDesc {
    selections: Vec<Selection>,
}

impl SelectionsDesc {
    pub fn new(s: &str) -> Option<Self> {
        let mut selections = Vec::new();
        for sel in s.split(' ') {
            let mut parts = sel.split(',');
            let start = parts.next()?;
            let end = parts.next()?;
            let mut start_parts = start.split('.');
            let start_row = start_parts.next()?;
            let start_col = start_parts.next()?;
            let mut end_parts = end.split('.');
            let end_row = end_parts.next()?;
            let end_col = end_parts.next()?;
            selections.push(Selection {
                start: SelectionPoint { row: start_row.parse().ok()?, column: start_col.parse().ok()? },
                end: SelectionPoint { row: end_row.parse().ok()?, column: end_col.parse().ok()? },
            });
        }
        Some(Self { selections })
    }

    pub fn extend_left(&mut self) -> usize {
        let mut min_col = None;
        for sel in &self.selections {
            match min_col {
                Some(min) => {
                    if sel.start.column < min {
                        min_col = Some(sel.start.column);
                    }
                }
                None => {
                    min_col = Some(sel.start.column);
                }
            }
        }
        let min_col = min_col.unwrap();

        for sel in &mut self.selections {
            sel.start.column = min_col;
        }
        min_col
    }
}

impl Into<String> for SelectionsDesc {
    fn into(self) -> String {
        let mut sels = Vec::new();
        for sel in self.selections {
            sels.push(format!("{}.{},{}.{}", sel.start.row, sel.start.column, sel.end.row, sel.end.column));
        }
        sels.join(" ")
    }
}

struct Selection {
    start: SelectionPoint,
    end: SelectionPoint,
}

struct SelectionPoint {
    row: usize,
    column: usize,
}

