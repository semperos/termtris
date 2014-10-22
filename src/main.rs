/*
 * Termtris -- Ported from the Lua written by Tyler Neylon
 *             See: https://github.com/tylerneylon/termtris
*/

use std::io::timer;
use std::time::duration::Duration;
use std::collections::TreeMap;

type ShapeLong = [[u8, ..4], ..1];
type ShapeMedium = [[u8, ..3], ..2];
type ShapeShort = [[u8, ..2], ..2];

enum Shape {
    LongShape(ShapeLong),
    MediumShape(ShapeMedium),
    ShortShape(ShapeShort),
}

static SHAPES : [Shape, ..7] = [
    MediumShape([[0,1,0],    //  #
                 [1,1,1]]),  // ###

    MediumShape([[0,1,1],    //  ##
                 [1,1,0]]),  // ##

    MediumShape([[1,1,0],   // ##
                 [0,1,1]]), //  ##

    LongShape([[1,1,1,1]]), // ####

    ShortShape([[1,1],      // ##
                [1,1]]),    // ##

    MediumShape([[1,0,0],   // #
                 [1,1,1]]), // ###

    MediumShape([[0,0,1],   //   #
                 [1,1,1]]), // ###
    ];

enum GameState {
    Unstarted,
    Playing,
    Paused,
    Over,
}
static mut GAME_STATE : GameState = Unstarted;

static BOARD_SIZE : [int, ..2] = [10,20];

enum BoardValue {
    Empty = 0,
    Border = -1,
}

/// Representation of board state, 0 = EMPTY, -1 = BORDER, everything else is a shape number based on index in SHAPES array.
// static mut BOARD : TreeMap = TreeMap::new();

/// Four possible rotations of a shape
enum Rotation {
    North = 0,
    East = 1,
    South = 2,
    West = 4,
}

/// Representation of a shape on the board with a certain rotation.
struct Piece {
    shape: Shape,
    rot: Rotation,
    x: uint,
    y: uint,
}

/// Current piece
static mut MOVING_PIECE : Piece = Piece {
    shape: MediumShape([[0,1,0],
                        [1,1,1]]),
    rot: North,
    x: 0,
    y: 0,
};

fn handle_input(stats: int, fall: int, next_piece: int) -> int {
    5
}

fn lower_piece_at_right_time(stats: int, fall:int , next_piece: int) -> int {
    6
}

fn draw_screen(stats: int, colors: int, next_piece: int) -> int {
    7
}

fn init_stats() -> int {
    1
}

fn init_fall() -> int {
    2
}

fn init_colors() -> int {
    3
}

fn init_next_piece() -> int {
    4
}

fn main() {
    let stats = init_stats();
    let fall = init_fall();
    let colors = init_colors();
    let next_piece = init_next_piece();
    println!("Initializations: {}", (stats, fall, colors, next_piece));

    loop {
        handle_input(stats, fall, next_piece);
        lower_piece_at_right_time(stats, fall, next_piece);
        draw_screen(stats, colors, next_piece);

        timer::sleep(Duration::milliseconds(16i64));
    }
}
