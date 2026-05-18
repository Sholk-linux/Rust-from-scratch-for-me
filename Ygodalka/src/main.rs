
use std::io;
use rand::Rng;

fn main() {
    println!("Угадай число от 1 до 100");
    let mut rng = rand::rng();
    let secret = rng.random_range(1..=100);
    let mut guess = String::new();
    let mut attemps = 0;

    loop {
        //получаем число
        guess.clear();
        io::stdin().read_line(&mut guess).unwrap();

        //выход
        let exit = guess.trim();
        if exit == "exit" {
            println!("Выход из игры. Загаданное число {} попыток {}", secret, attemps);
            break;
        }

        //записываем и проверяем
        let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Цифру надо писать!");
            continue;
            }
        };

        //больше меньше выигрыш
        if guess == secret {
            println!("Да ты угадал за {} попытки", attemps);
            break
        } else if guess > secret {
            println!("Это число меньше");
            attemps += 1;
        } else if guess < secret {
            println!("Это число больше");
            attemps += 1;
        }
    }
}
