use art_intell::Game;
use rand::Rng;
use std::io;
//use std::rc::Rc;

fn main() {
    let mut game = Game::new();
    println!("This is the board\n1|2|3\n4|5|6\n7|8|9");
    let player = rand::thread_rng().gen_range(1,3);
    println!("You are PLAYER{}", player);
    while game.turn < 10{
        let mut s = "".to_string();
        match player {
            1 => {
                println!("Choose the no where you want to place X");
                io::stdin().read_line(&mut s).unwrap();
                let n = s.trim().parse().unwrap();
                game.go(n);
                match game.turn {
                    1 => game.go(1),
                    2 => match game.board[4]{
                        2 => game.go(5),
                        _ => game.go(1)
                    }
                    3 => match game.board[8] {
                        2 => game.go(9),
                        _ => game.go(3),
                    }
                    4 => match game.posswin(1) {
                        0 => game.go(game.make_two()),
                        _ => game.go(game.posswin(1))
                    }
                    5 => if game.posswin(1)!=0{
                        game.go(game.posswin(1))
                    }
                    else if game.posswin(2)!=0 {
                        game.go(game.posswin(2))
                    }
                    else if game.board[6]==2 {
                        game.go(7)
                    }
                    else {
                        game.go(3)
                    }
                    6 => if game.posswin(2)!=0 {
                        game.go(game.posswin(2))
                    }
                    else if game.posswin(1)!=0 {
                        game.go(game.posswin(1))
                    }
                    else {
                        game.go(game.make_two())
                    }
                    7 => if game.posswin(1)!=0 {
                        game.go(game.posswin(1))
                    }
                    else if game.posswin(2)!=0 {
                        game.go(game.posswin(2))
                    }
                    else {
                        let mut blank;
                        let arr = game.board;
                        for (a,b) in arr.iter().enumerate(){
                            match b {
                                2 => blank = a,
                                _ => continue,
                            }
                            game.go(blank)
                        }
                    }
                    8 => if game.posswin(2)!=0 {
                        game.go(game.posswin(2))
                    }
                    else if game.posswin(1)!=0 {
                        game.go(game.posswin(1))
                    }
                    else {
                        let mut blank;
                        let arr = game.board;
                        for (a,b) in arr.iter().enumerate(){
                            match b {
                                2 => blank = a,
                                _ => continue,
                            }
                            game.go(blank)
                        }
                    }
                    9 => if game.posswin(1)!=0 {
                        game.go(game.posswin(1))
                    }
                    else if game.posswin(2)!=0 {
                        game.go(game.posswin(2))
                    }
                    else {
                        let mut blank;
                        let arr = game.board;
                        for (a,b) in arr.iter().enumerate(){
                            match b {
                                2 => blank = a,
                                _ => continue,
                            }
                            game.go(blank)
                        }
                    }
                    _ => println!("Game Over")
                };
            },
            2 => {
                match game.turn {
                    1 => game.go(1),
                    2 => match game.board[4]{
                        2 => game.go(5),
                        _ => game.go(1)
                    }
                    3 => match game.board[8] {
                        2 => game.go(9),
                        _ => game.go(3),
                    }
                    4 => match game.posswin(1) {
                        0 => game.go(game.make_two()),
                        _ => game.go(game.posswin(1))
                    }
                    5 => if game.posswin(1)!=0{
                        game.go(game.posswin(1))
                    }
                    else if game.posswin(2)!=0 {
                        game.go(game.posswin(2))
                    }
                    else if game.board[6]==2 {
                        game.go(7)
                    }
                    else {
                        game.go(3)
                    }
                    6 => if game.posswin(2)!=0 {
                        game.go(game.posswin(2))
                    }
                    else if game.posswin(1)!=0 {
                        game.go(game.posswin(1))
                    }
                    else {
                        game.go(game.make_two())
                    }
                    7 => if game.posswin(1)!=0 {
                        game.go(game.posswin(1))
                    }
                    else if game.posswin(2)!=0 {
                        game.go(game.posswin(2))
                    }
                    else {
                        let mut blank;
                        let arr = game.board;
                        for (a,b) in arr.iter().enumerate(){
                            match b {
                                2 => blank = a,
                                _ => continue,
                            }
                            game.go(blank)
                        }
                    }
                    8 => if game.posswin(2)!=0 {
                        game.go(game.posswin(2))
                    }
                    else if game.posswin(1)!=0 {
                        game.go(game.posswin(1))
                    }
                    else {
                        let mut blank;
                        let arr = game.board;
                        for (a,b) in arr.iter().enumerate(){
                            match b {
                                2 => blank = a,
                                _ => continue,
                            }
                            game.go(blank)
                        }
                    }
                    9 => if game.posswin(1)!=0 {
                        game.go(game.posswin(1))
                    }
                    else if game.posswin(2)!=0 {
                        game.go(game.posswin(2))
                    }
                    else {
                        let mut blank;
                        let arr = game.board;
                        for (a,b) in arr.iter().enumerate(){
                            match b {
                                2 => blank = a,
                                _ => continue,
                            }
                            game.go(blank)
                        }
                    }
                    _ => println!("Game Over")
                };
                println!("Choose the no where you want to place O");
                io::stdin().read_line(&mut s).unwrap();
                let n = s.trim().parse().unwrap();
                game.go(n);
            },
            _ => println!("Not a valid player"),
        }
    }
}
