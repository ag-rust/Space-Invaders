use size::*;
use max_min::*;
use entity::*;
use point::*;
use color::*;

pub struct GridDistribution {
    pub available_space: Size,
    pub entity_size: Size,
    pub vertical_padding: u32,
    pub horizontal_padding: u32,
}

impl GridDistribution
{
    pub fn distribute(&self) -> Vec<Point> {
        /*
         * how many per row?
         */

        let count_horizontally = self.how_many_to_fill_space(
            self.entity_size.width,
            self.horizontal_padding,
            self.available_space.width,
            );

        let count_vertically = self.how_many_to_fill_space(
            self.entity_size.height,
            self.vertical_padding,
            self.available_space.height,
            );

        let mut grid: Vec<Point> = Vec::new();

        let mut space_used_vertically = 0;
        let mut space_used_horizontally = 0;

        for row in 0..count_vertically {
            for col in 0..count_horizontally {
                let point = Point {
                    x: space_used_horizontally as i32,
                    y: space_used_vertically as i32,
                };
                grid.push(point);
                space_used_horizontally += self.entity_size.width + self.horizontal_padding;
            }
            space_used_horizontally = 0;
            space_used_vertically += self.entity_size.height + self.vertical_padding;
        }

        grid
    }

    fn how_many_to_fill_space(&self, size: u32, padding: u32, available_space: u32) -> u32 {
        let mut count = 0;

        loop {
            let space_required = (size + padding) * count;
            let remaining_space = (available_space as i32) - space_required as i32;

            if remaining_space >= 0 {
                count += 1;
            } else {
                println!("remaining_space: {}", remaining_space);

                if space_required - padding <= available_space {
                    return count as u32
                } else {
                    return (count - 1) as u32
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tiny_grid() {
        let grid = GridDistribution {
            available_space: Size { height: 1, width: 1 },
            entity_size: Size { height: 1, width: 1 },
            vertical_padding: 0,
            horizontal_padding: 0,
        }.distribute();

        assert_eq!(grid.len(), 1);
        assert_includes(&grid, Point { x: 0, y: 0 });
    }

    #[test]
    fn slightly_bigger_grid() {
        let grid = GridDistribution {
            available_space: Size { height: 2, width: 2 },
            entity_size: Size { height: 1, width: 1 },
            vertical_padding: 0,
            horizontal_padding: 0,
        }.distribute();

        assert_eq!(grid.len(), 4);
        assert_includes(&grid, Point { x: 0, y: 0 });
        assert_includes(&grid, Point { x: 0, y: 1 });
        assert_includes(&grid, Point { x: 1, y: 0 });
        assert_includes(&grid, Point { x: 1, y: 1 });
    }

    #[test]
    fn grid_with_padding() {
        let grid = GridDistribution {
            available_space: Size { height: 1, width: 5 },
            entity_size: Size { height: 1, width: 1 },
            vertical_padding: 0,
            horizontal_padding: 1,
        }.distribute();

        assert_eq!(grid.len(), 3);
        println!("{:?}", grid);
        assert_includes(&grid, Point { x: 0, y: 0 });
        assert_includes(&grid, Point { x: 2, y: 0 });
        assert_includes(&grid, Point { x: 4, y: 0 });
    }

    #[test]
    fn two_d_grid_with_padding() {
        let grid = GridDistribution {
            available_space: Size { height: 5, width: 5 },
            entity_size: Size { height: 1, width: 1 },
            vertical_padding: 1,
            horizontal_padding: 1,
        }.distribute();

        assert_eq!(grid.len(), 9);
        assert_includes(&grid, Point { x: 0, y: 0 });
        assert_includes(&grid, Point { x: 2, y: 0 });
        assert_includes(&grid, Point { x: 4, y: 0 });
        assert_includes(&grid, Point { x: 0, y: 2 });
        assert_includes(&grid, Point { x: 2, y: 2 });
        assert_includes(&grid, Point { x: 4, y: 2 });
        assert_includes(&grid, Point { x: 0, y: 4 });
        assert_includes(&grid, Point { x: 2, y: 4 });
        assert_includes(&grid, Point { x: 4, y: 4 });
    }

    fn assert_includes<T: Eq>(vec: &Vec<T>, x: T) {
        assert!(vec_includes(vec, x));
    }

    fn vec_includes<T: Eq>(vec: &Vec<T>, x: T) -> bool {
        for y in vec {
            if &x == y { return true }
        }
        false
    }
}
