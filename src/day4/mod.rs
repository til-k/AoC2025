#[derive(Clone)]
struct CharField {
    width: i64,
    height: i64,
    data: Box<[u8]>
}
#[derive(Copy, Clone)]
struct CharFieldCursor {
    x: i64,
    y: i64,
    c: char
}
struct HorizotalFullIterator<'a>{
    position: usize,
    field: &'a CharField,
}
struct NeighborhoodIterator<'a>{
    n_index : usize,
    center_position: CharFieldCursor,
    field: &'a CharField,
}
impl CharField {
    // init field assuming input is a quadratic field of characters, with new lines as horizontal seperator
    fn new_from_quad_str(input: &str) -> Option<Self> {
        let height = input.lines().count();
        if let Some(first_line) = input.lines().next() {
            let width = first_line.len();
            return Some(Self {
                width: width as i64,
                height: height as i64,
                data: input.lines().collect::<Vec<&str>>().join("").as_bytes().into(),
            });
        }
        None
    }

    fn set_to(&mut self, cfc: CharFieldCursor) {
        self.data[(cfc.y * self.width + cfc.x) as usize] = cfc.c as u8;
    }
    
    fn iter_horizontally(&'_ self) -> HorizotalFullIterator<'_> {
        HorizotalFullIterator {
            position: 0,
            field: self,
        }
    }

    fn iter_iterator_neighorhood(&'_ self, cfc: CharFieldCursor) -> NeighborhoodIterator<'_> {
        NeighborhoodIterator {
            n_index: 0 ,
            center_position: cfc,
            field : self,
        }
    }
}

impl Iterator for HorizotalFullIterator<'_> {
    type Item = CharFieldCursor;

    fn next(&mut self) -> Option<Self::Item> {
        let b = self.field.data.get(self.position);
        self.position += 1;
        if let Some(c) = b {
            Some(CharFieldCursor{
                x: (self.position as i64 - 1) % self.field.width,
                y: (self.position as i64 - 1) / self.field.width,
                c: *c as char
            })
        }
        else {None}
    }
}


/*
0 1 2
3 X 4
5 6 7
*/
impl Iterator for NeighborhoodIterator<'_> {
    type Item = Option<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n_index < 8 {
            Some( {
                    let x_offset = {
                        match self.n_index {
                            0 | 3 | 5 => -1,
                            1 | 6 => 0,
                            2 | 4 | 7 => 1,
                            _ => 0
                        }
                    };
                    let y_offset = {
                        match self.n_index {
                            0 | 1 | 2 => -1,
                            3 | 4 => 0,
                            5 | 6 | 7 => 1,
                            _ => 0
                        }
                    };
                    self.n_index += 1;

                    let x = self.center_position.x - x_offset;
                    let y = self.center_position.y - y_offset;
                    if (x < 0) | (y < 0) | (x >= self.field.width) | (y >= self.field.height) {
                        None
                    } 
                    else {
                        let neighbor_position = y * self.field.height + x;
                        if neighbor_position > 0 {
                            let b = self.field.data.get(neighbor_position as usize);
                            if let Some(c) = b {Some(*c as char)}
                            else {None}
                        }
                        else {None}
                    }
                }
                )
            }
        else {
            None
        }

    }
}

pub struct Day4;
impl crate::DailyRiddle for Day4 {
    fn name(&self) -> &str {
        "Day 4"
    }

    fn part1(&self) -> (i128, i128) {
        return (
            solve1(include_str!("sample.txt")),
            solve1(include_str!("riddle.txt"))
        );
    }

    fn part2(&self) -> (i128, i128) {
        return (
            solve2(include_str!("sample.txt")),
            solve2(include_str!("riddle.txt")),
        );
    }
}

fn solve1(input: &str) -> i128 {
    if let Some(field) = CharField::new_from_quad_str(input) {
        let mut solution: i128 = 0;
        let mut full_iter = field.iter_horizontally();
        while let Some(cfc) = full_iter.next() {
            if cfc.c == '@' {    
                let mut neighbor_count = 0;
                let mut neighborhood_iter = field.iter_iterator_neighorhood(cfc);
                while let Some(n) = neighborhood_iter.next() {
                    if let Some(n_char) = n {
                        if n_char == '@' {
                            neighbor_count += 1;
                        }
                    }
                }
                if neighbor_count < 4 {
                    solution += 1;
                }
            }
        }
        return solution;
    } else {
        return 0;
    }
}

fn solve2(input: &str) -> i128 {
    if let Some(initial_field) = CharField::new_from_quad_str(input) {
        let mut field_was_reduceable= true;
        let mut solution: i128 = 0;
        let mut current_field = initial_field.clone();
        while field_was_reduceable {
            let mut next_field = current_field.clone();
            let mut full_iter = current_field.iter_horizontally();
            field_was_reduceable = false;
            while let Some(cfc) = full_iter.next() {
                if cfc.c == '@' {    
                    let mut neighbor_count = 0;
                    let mut neighborhood_iter = current_field.iter_iterator_neighorhood(cfc);
                    while let Some(n) = neighborhood_iter.next() {
                        if let Some(n_char) = n {
                            if n_char == '@' {
                                neighbor_count += 1;
                            }
                        }
                    }
                    if neighbor_count < 4 {
                        next_field.set_to(CharFieldCursor { x: cfc.x, y: cfc.y, c: '.' });
                        solution += 1;
                        field_was_reduceable = true;
                    }
                }
            }
            current_field = next_field;
        }
        return solution;
    } else {
        return 0;
    }
}
