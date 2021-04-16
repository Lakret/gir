use mazes::maze::Maze;

#[test]
fn maze_generator_works() {
  let maze = Maze::generate(10, 10);
  assert_eq!(maze.width(), 10);
  assert_eq!(maze.height(), 10);
  assert!(!maze[(0, 0)].is_empty());
  assert!(!maze[(0, 1)].is_empty());
  assert!(!maze[(9, 9)].is_empty());
}
