extern crate rand;
use rand::Rng;
#[derive(Debug,Default)]
pub struct Game{
    pub board: [u32;9],
    pub turn: u32,
}

impl Game {
    pub fn new()->Game{
        Game{board: [2; 9], turn: 1}
    }

    pub fn make_two(&self) -> usize {
        if self.board[4] == 2 {
            5
        } else {
            let mut temp_vec = Vec::new();
            for i in 0..4 {
                if self.board[2 * i+1] == 2 {
                    temp_vec.push(2 * i+2);
                }
            }
            let rand = rand::thread_rng().gen_range(0, temp_vec.len());
            //dbg!(&temp_vec);
            temp_vec[rand]
        }
    }

    pub fn posswin(&self, player: u32) -> usize {
        for i in 0..3 {
            if player == 1 {
                if self.board[3*i] * self.board[3*i + 1] * self.board[3*i + 2] == 18 {
                    if self.board[i] == 2 {
                        return i+1
                    } else if self.board[i + 1] == 2 {
                        return i + 2
                    } else {
                        return i + 3
                    }
                } else if self.board[i] * self.board[i + 3] * self.board[i + 6] == 18 {
                    if self.board[i] == 2 {
                        return i+1
                    } else if self.board[i + 3] == 2 {
                        return i + 4
                    } else {
                        return i + 7
                    }
                } else if self.board[0] * self.board[4] * self.board[8] == 18 {
                    if self.board[0] == 2 {
                        return 1
                    } else if self.board[4] == 2 {
                        return 5
                    } else {
                        return 9
                    }
                } else if self.board[2] * self.board[4] * self.board[6] == 18 {
                    if self.board[2] == 2 {
                        return 3
                    } else if self.board[4] == 2 {
                        return 5
                    } else {
                        return 7
                    }
                }
            }
            if player == 2 {
                if self.board[3*i] * self.board[3*i + 1] * self.board[3*i + 2] == 50 {
                    if self.board[i] == 2 {
                        return i+1
                    } else if self.board[i + 1] == 2 {
                        return i + 2
                    } else {
                        return i + 3
                    }
                }
                if self.board[i] * self.board[i + 3] * self.board[i + 6] == 50 {
                    if self.board[i] == 2 {
                        return i+1
                    }
                    if self.board[i + 3] == 2 {
                        return i + 4
                    } else {
                        return i + 7
                    }
                }
                if self.board[0] * self.board[4] * self.board[8] == 50 {
                    if self.board[0] == 2 {
                        return 1
                    }
                    if self.board[4] == 2 {
                        return 5
                    } else {
                        return 9
                    }
                } else if self.board[2] * self.board[4] * self.board[6] == 50 {
                    if self.board[2] == 2 {
                        return 3
                    } else if self.board[4] == 2 {
                        return 5
                    } else {
                        return 7
                    }
                } else {
                    return 0
                }
            }
        }
            0
    }

    pub fn go(&mut self, n: usize) {
        if self.turn % 2 == 1 {
            self.board[n - 1] = 3;
            //self.player = 1;
        } else {
            self.board[n - 1] = 5;
            //self.player = 2;

        }
        &self.check_win();
        //dbg!(self.board);
        println!("AI moved to {}", n);
        self.turn+=1;
    }

    pub fn check_win(&self){
        for i in 0..3 {
            {
                if self.board[3 * i] * self.board[3 * i + 1] * self.board[3 * i + 2] == 27 {
                    println!("Player1 wins");
                    break;
                } else if self.board[i] * self.board[i + 3] * self.board[i + 6] == 27 {
                    println!("Player1 wins");
                    break;
                    break;
                } else if self.board[0] * self.board[4] * self.board[8] == 27 {
                    println!("Player1 wins");
                    break;
                } else if self.board[2] * self.board[4] * self.board[6] == 27 {
                    println!("Player1 wins");
                    break;
                }
                if self.board[3 * i] * self.board[3 * i + 1] * self.board[3 * i + 2] == 125 {
                    println!("Player2 wins");
                    break;
                }
                if self.board[i] * self.board[i + 3] * self.board[i + 6] == 125 {
                    println!("Player2 wins");
                    break;
                }
                if self.board[0] * self.board[4] * self.board[8] == 125 {
                    println!("Player2 wins");
                    break;
                } else if self.board[2] * self.board[4] * self.board[6] == 125 {
                    println!("Player2 wins");
                    break;
                }
            }
        }
    }
}
