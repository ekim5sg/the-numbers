// src/app/questions.rs
//
// Kid-friendly question bank for Grades 1–5.
// Each day provides Core questions + Stretch questions (gentle challenge).
//
// NOTE: Names and comments intentionally avoid scary academic labels.
// The UI uses titles/subtitles from data.rs.

use super::grade::Grade;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Difficulty {
    Core,
    Stretch,
}

#[derive(Clone, Debug)]
pub struct Question {
    pub prompt: &'static str,
    pub answer: &'static str,
    pub difficulty: Difficulty,
}

pub fn questions_for(day_id: usize, grade: Grade) -> Vec<Question> {
    match day_id {
        1 => day1_patterns_and_change(grade),
        2 => day2_missing_number_puzzles(grade),
        3 => day3_shapes_around_us(grade),
        4 => day4_angles_and_turns(grade),
        5 => day5_groups_and_averages(grade),
        6 => day6_arrows_and_grids(grade),
        7 => day7_what_might_happen(grade),
        8 => day8_number_secrets(grade),
        9 => day9_counting_smart(grade),
        10 => day10_how_things_change(grade),
        11 => day11_how_we_know(grade),
        12 => day12_choosing_wisely(grade),
        _ => vec![q("Try another day!", "ok", Difficulty::Core)],
    }
}

/* ---------------------------
   DAY 1 — Patterns & Change
---------------------------- */
fn day1_patterns_and_change(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("What comes next? 1, 2, 3, __", "4", Difficulty::Core),
            q("What comes next? 2, 4, 6, __", "8", Difficulty::Core),
            q("Stretch: You hop 2 spaces each time. Starting at 0, where are you after 3 hops?", "6", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("What comes next? 5, 10, 15, __", "20", Difficulty::Core),
            q("Count down: 20, 18, 16, __", "14", Difficulty::Core),
            q("Stretch: You earn 3 points each turn. If you start at 1, what after 4 turns?", "13", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("What comes next? 3, 6, 9, __", "12", Difficulty::Core),
            q("How much did it change? 12 → 19", "7", Difficulty::Core),
            q("Stretch: A plant grows 2 cm each day. How much in 5 days?", "10", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("How much did it change? 45 → 60", "15", Difficulty::Core),
            q("What comes next? 100, 90, 80, __", "70", Difficulty::Core),
            q("Stretch: You save $4 per week. How much after 6 weeks?", "24", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("How much did it change? 2.5 → 4.0", "1.5", Difficulty::Core),
            q("What comes next? 1, 4, 9, 16, __", "25", Difficulty::Core),
            q("Stretch: A car goes 30 miles in 1 hour. How far in 2.5 hours?", "75", Difficulty::Stretch),
        ],
    }
}

/* ---------------------------
   DAY 2 — Find the Missing Number
---------------------------- */
fn day2_missing_number_puzzles(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("3 + __ = 5", "2", Difficulty::Core),
            q("__ + 4 = 7", "3", Difficulty::Core),
            q("Stretch: 9 - __ = 6", "3", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("__ + 6 = 10", "4", Difficulty::Core),
            q("12 - __ = 5", "7", Difficulty::Core),
            q("Stretch: 3 × __ = 12", "4", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("__ + 14 = 20", "6", Difficulty::Core),
            q("36 ÷ __ = 6", "6", Difficulty::Core),
            q("Stretch: 2 × __ + 3 = 11", "4", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("x + 18 = 40. x = ?", "22", Difficulty::Core),
            q("5x = 35. x = ?", "7", Difficulty::Core),
            q("Stretch: 3x + 2 = 20. x = ?", "6", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("x - 12 = 19. x = ?", "31", Difficulty::Core),
            q("4x = 48. x = ?", "12", Difficulty::Core),
            q("Stretch: 2x + 5 = 29. x = ?", "12", Difficulty::Stretch),
        ],
    }
}

/* ---------------------------
   DAY 3 — Shapes Around Us
---------------------------- */
fn day3_shapes_around_us(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("How many sides does a triangle have?", "3", Difficulty::Core),
            q("How many corners does a rectangle have?", "4", Difficulty::Core),
            q("Stretch: A square has 4 sides. Are they all the same length? (yes/no)", "yes", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("How many sides does a hexagon have?", "6", Difficulty::Core),
            q("A rectangle has how many sides total?", "4", Difficulty::Core),
            q("Stretch: A shape with 8 sides is an __.", "octagon", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("Perimeter: a square has side 5. Perimeter = ?", "20", Difficulty::Core),
            q("Perimeter: a rectangle is 3 by 7. Perimeter = ?", "20", Difficulty::Core),
            q("Stretch: Area: a rectangle is 4 by 6. Area = ?", "24", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("Area: a rectangle is 8 by 3. Area = ?", "24", Difficulty::Core),
            q("Perimeter: rectangle 10 by 2. Perimeter = ?", "24", Difficulty::Core),
            q("Stretch: A right triangle has legs 3 and 4. Long side = ?", "5", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("Area: a triangle has base 10 and height 6. Area = ?", "30", Difficulty::Core),
            q("A circle’s distance across is called the __.", "diameter", Difficulty::Core),
            q("Stretch: A right triangle has legs 6 and 8. Long side = ?", "10", Difficulty::Stretch),
        ],
    }
}

/* ---------------------------
   DAY 4 — Angles & Turns
---------------------------- */
fn day4_angles_and_turns(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("A half turn is like making a big U-turn. Is it bigger than a quarter turn? (yes/no)", "yes", Difficulty::Core),
            q("A straight line is like a straight turn. Is it 0 turns or a straight turn? (straight)", "straight", Difficulty::Core),
            q("Stretch: A quarter turn is like the corner of a square. How many quarter turns make a full turn?", "4", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("How many quarter turns make a full turn?", "4", Difficulty::Core),
            q("Is a right angle like an L shape? (yes/no)", "yes", Difficulty::Core),
            q("Stretch: Two right angles together make a __ turn (half/full).", "half", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("A right angle is 90 degrees. Write the number: __", "90", Difficulty::Core),
            q("A straight angle is 180 degrees. Write the number: __", "180", Difficulty::Core),
            q("Stretch: Two right angles together is __ degrees.", "180", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("A full turn is 360 degrees. Write the number: __", "360", Difficulty::Core),
            q("An angle smaller than 90 degrees is called __ (acute/right/obtuse).", "acute", Difficulty::Core),
            q("Stretch: 360 ÷ 4 = ?", "90", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("What is 180 ÷ 2?", "90", Difficulty::Core),
            q("An angle bigger than 90 but less than 180 is called __ (obtuse).", "obtuse", Difficulty::Core),
            q("Stretch: If you turn 45 degrees 4 times, how many degrees is that?", "180", Difficulty::Stretch),
        ],
    }
}

/* ---------------------------
   DAY 5 — Making Sense of Numbers (Groups & Averages)
---------------------------- */
fn day5_groups_and_averages(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("Which number shows up the most? 2, 3, 2, 1", "2", Difficulty::Core),
            q("What is the biggest number? 5, 1, 4", "5", Difficulty::Core),
            q("Stretch: Put in order (small to big): 3, 1, 2. Write as 1,2,3", "1,2,3", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("Which number shows up the most? 4, 2, 4, 3, 4", "4", Difficulty::Core),
            q("What is the middle number? 1, 3, 5", "3", Difficulty::Core),
            q("Stretch: Put in order: 6, 2, 4 (write 2,4,6)", "2,4,6", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("Mean (average): (2 + 4 + 6) ÷ 3 = ?", "4", Difficulty::Core),
            q("Median of 2, 9, 5 (in order 2,5,9) is ?", "5", Difficulty::Core),
            q("Stretch: Mode of 1, 2, 2, 3, 3, 3 is ?", "3", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("Mean: (10 + 20 + 30) ÷ 3 = ?", "20", Difficulty::Core),
            q("Median of 4, 8, 1, 9, 2 is ? (order 1,2,4,8,9)", "4", Difficulty::Core),
            q("Stretch: Range of 2, 10, 7, 5 is ? (max-min)", "8", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("Mean: (6 + 7 + 9 + 8) ÷ 4 = ?", "7.5", Difficulty::Core),
            q("Range of 3, 12, 8, 5 is ?", "9", Difficulty::Core),
            q("Stretch: Mean: (1.5 + 2.5 + 3.0) ÷ 3 = ?", "2.333", Difficulty::Stretch),
        ],
    }
}

/* ---------------------------
   DAY 6 — Arrows & Grids (Movement)
---------------------------- */
fn day6_arrows_and_grids(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("On a grid, if you move 1 step right, did you go left or right?", "right", Difficulty::Core),
            q("If you move 2 steps up, did you go up or down?", "up", Difficulty::Core),
            q("Stretch: Start at 0. Move +2, then +3. Where are you now?", "5", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("Start at 0. Move +5. Where are you?", "5", Difficulty::Core),
            q("Start at 7. Move -3. Where are you?", "4", Difficulty::Core),
            q("Stretch: Start at 2. Move +4, then -1. Where are you?", "5", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("Coordinates: Which comes first (x,y): left/right or up/down? (x/y)", "x", Difficulty::Core),
            q("Start at 10. Move -6. Where are you?", "4", Difficulty::Core),
            q("Stretch: Start at 3. Move +7, then -4. Where are you?", "6", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("A point (2,5) has x = ? ", "2", Difficulty::Core),
            q("A point (2,5) has y = ? ", "5", Difficulty::Core),
            q("Stretch: Start at 0. Move +12, then -5, then +3. Where are you?", "10", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("Start at -2. Move +7. Where are you?", "5", Difficulty::Core),
            q("Start at 6. Move -9. Where are you?", "-3", Difficulty::Core),
            q("Stretch: A point (x,y) = (4, -1). y = ?", "-1", Difficulty::Stretch),
        ],
    }
}

/* ---------------------------
   DAY 7 — What Might Happen? (Chance)
---------------------------- */
fn day7_what_might_happen(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("Which is more likely: the sun rises tomorrow OR it snows candy? (sun/candy)", "sun", Difficulty::Core),
            q("If a bag has 10 red and 1 blue, which color is easier to grab? (red/blue)", "red", Difficulty::Core),
            q("Stretch: Is it possible to roll a 7 on one standard die? (yes/no)", "no", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("A coin has heads and tails. How many sides? (2)", "2", Difficulty::Core),
            q("More likely on a die: roll a 1 OR roll a 7? (1/7)", "1", Difficulty::Core),
            q("Stretch: Is rolling an even number possible on a die? (yes/no)", "yes", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("A fair coin: chance of heads is 1 out of __.", "2", Difficulty::Core),
            q("A die: how many outcomes? (1–6) total = __", "6", Difficulty::Core),
            q("Stretch: On a die, probability of rolling a 6 as a fraction is 1/__", "6", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("On a die, probability of rolling an even number is __/6 (write fraction)", "3/6", Difficulty::Core),
            q("On a die, probability of rolling >4 is __/6", "2/6", Difficulty::Core),
            q("Stretch: Simplify 3/6 to __ (1/2)", "1/2", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("A bag has 3 red and 2 blue. Probability of red is __/5", "3/5", Difficulty::Core),
            q("A die: probability of rolling 1 or 2 is __/6", "2/6", Difficulty::Core),
            q("Stretch: Simplify 2/6 to __ (1/3)", "1/3", Difficulty::Stretch),
        ],
    }
}

/* ---------------------------
   DAY 8 — Number Secrets
---------------------------- */
fn day8_number_secrets(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("Is 6 even or odd? (even/odd)", "even", Difficulty::Core),
            q("Is 7 even or odd? (even/odd)", "odd", Difficulty::Core),
            q("Stretch: Skip count by 2s: 2, 4, 6, __", "8", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("Is 12 even or odd? (even/odd)", "even", Difficulty::Core),
            q("Is 15 even or odd? (even/odd)", "odd", Difficulty::Core),
            q("Stretch: Skip count by 5s: 5, 10, 15, __", "20", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("Is 21 divisible by 3? (yes/no)", "yes", Difficulty::Core),
            q("Is 20 divisible by 3? (yes/no)", "no", Difficulty::Core),
            q("Stretch: Is 29 a prime number? (yes/no)", "yes", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("Is 27 divisible by 9? (yes/no)", "yes", Difficulty::Core),
            q("Is 28 divisible by 7? (yes/no)", "yes", Difficulty::Core),
            q("Stretch: Is 49 prime? (yes/no)", "no", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("What is a factor of 24? (pick: 2/5) Answer with 2 or 5.", "2", Difficulty::Core),
            q("Is 35 divisible by 5? (yes/no)", "yes", Difficulty::Core),
            q("Stretch: Is 31 prime? (yes/no)", "yes", Difficulty::Stretch),
        ],
    }
}

/* ---------------------------
   DAY 9 — Counting Smart
---------------------------- */
fn day9_counting_smart(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("You have 2 shirts and 1 hat. How many outfits? (2)", "2", Difficulty::Core),
            q("You can choose 1 snack: apple or banana. How many choices? (2)", "2", Difficulty::Core),
            q("Stretch: You have 2 shirts and 2 hats. How many outfits?", "4", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("2 shirts and 2 pants. How many outfits?", "4", Difficulty::Core),
            q("3 snacks to choose from. How many choices?", "3", Difficulty::Core),
            q("Stretch: 3 shirts and 2 hats. How many outfits?", "6", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("If you have 3 shirts and 2 pants, outfits = ?", "6", Difficulty::Core),
            q("How many ways to pick 1 of 4 prizes?", "4", Difficulty::Core),
            q("Stretch: A set has 2 items. How many subsets? (include empty set)", "4", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("A set has 3 items. How many subsets? (include empty set)", "8", Difficulty::Core),
            q("You can go Left or Right at 2 turns. How many paths? (2×2)", "4", Difficulty::Core),
            q("Stretch: A set has 4 items. How many subsets?", "16", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("A set has 4 items. How many subsets?", "16", Difficulty::Core),
            q("If you have 5 choices for topping, how many ways to pick 1 topping?", "5", Difficulty::Core),
            q("Stretch: A set has 5 items. How many subsets?", "32", Difficulty::Stretch),
        ],
    }
}

/* ---------------------------
   DAY 10 — How Things Change (Rates)
---------------------------- */
fn day10_how_things_change(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("You get 1 sticker each day. After 5 days, stickers = ?", "5", Difficulty::Core),
            q("You lose 1 point each turn from 6. After 2 turns, points = ?", "4", Difficulty::Core),
            q("Stretch: You gain 2 points each turn from 1. After 3 turns, points = ?", "7", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("You save $2 each week. After 4 weeks, money = ?", "8", Difficulty::Core),
            q("A candle burns down 1 inch per hour from 6 inches. After 2 hours, inches = ?", "4", Difficulty::Core),
            q("Stretch: You grow 3 cm per year (pretend). In 5 years, cm = ?", "15", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("A game adds 4 points each round. After 3 rounds, points added = ?", "12", Difficulty::Core),
            q("A robot moves 2 steps each turn. After 6 turns, steps = ?", "12", Difficulty::Core),
            q("Stretch: Start at 10 and subtract 3 each time. After 4 times, number = ?", "-2", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("A bike goes 5 miles per hour. In 3 hours, miles = ?", "15", Difficulty::Core),
            q("A tank fills 4 liters per minute. In 5 minutes, liters = ?", "20", Difficulty::Core),
            q("Stretch: If you travel 30 miles in 2 hours, miles per hour = ?", "15", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("A car goes 60 miles in 2 hours. Miles per hour = ?", "30", Difficulty::Core),
            q("You read 12 pages in 3 days. Pages per day = ?", "4", Difficulty::Core),
            q("Stretch: If a plant grows 2.5 cm per week, in 4 weeks it grows __ cm.", "10", Difficulty::Stretch),
        ],
    }
}

/* ---------------------------
   DAY 11 — How We Know (Truth & Reason)
---------------------------- */
fn day11_how_we_know(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("True or false: 2 + 2 = 4 (true/false)", "true", Difficulty::Core),
            q("True or false: 5 is smaller than 3 (true/false)", "false", Difficulty::Core),
            q("Stretch: If you have 3 cookies and eat 1, do you have 2 left? (yes/no)", "yes", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("True or false: 10 - 7 = 3 (true/false)", "true", Difficulty::Core),
            q("True or false: 4 + 4 = 10 (true/false)", "false", Difficulty::Core),
            q("Stretch: If all squares have 4 sides, does a square have 4 sides? (yes/no)", "yes", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("If 6 is even, then it ends in 0,2,4,6,8. Does 6 end in 0,2,4,6,8? (yes/no)", "yes", Difficulty::Core),
            q("True or false: 9 is divisible by 3 (true/false)", "true", Difficulty::Core),
            q("Stretch: If a number ends in 0, is it divisible by 10? (yes/no)", "yes", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("If a shape has 3 sides, it is a triangle. A triangle has 3 sides. Is that true? (yes/no)", "yes", Difficulty::Core),
            q("True or false: 12 is divisible by 5 (true/false)", "false", Difficulty::Core),
            q("Stretch: If a number is divisible by 2, is it even? (yes/no)", "yes", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("True or false: If a number is divisible by 2, it must be even (true/false)", "true", Difficulty::Core),
            q("True or false: 25 is divisible by 4 (true/false)", "false", Difficulty::Core),
            q("Stretch: If x = 5, then x + 3 = ? (write number)", "8", Difficulty::Stretch),
        ],
    }
}

/* ---------------------------
   DAY 12 — Choosing Wisely (Strategy)
---------------------------- */
fn day12_choosing_wisely(grade: Grade) -> Vec<Question> {
    match grade {
        Grade::G1 => vec![
            q("If you want more candy, choose the bigger number: 3 or 5 (answer 3/5)", "5", Difficulty::Core),
            q("Best move: If you’re at 8 and want to reach 10, add 2 or add 5? (2/5)", "2", Difficulty::Core),
            q("Stretch: Choose the best: to make 10 from 7, add 3 or add 4? (3/4)", "3", Difficulty::Stretch),
        ],
        Grade::G2 => vec![
            q("Best choice: to make 12 from 9, add 3 or add 5? (3/5)", "3", Difficulty::Core),
            q("Which is better for reaching 20 faster: add 10 or add 2? (10/2)", "10", Difficulty::Core),
            q("Stretch: To land exactly on 15 from 11, add 4 or add 5? (4/5)", "4", Difficulty::Stretch),
        ],
        Grade::G3 => vec![
            q("If you can pick 1 prize: 100 points or 20 points (100/20)", "100", Difficulty::Core),
            q("To reach 30 from 24, add 6 or add 8? (6/8)", "6", Difficulty::Core),
            q("Stretch: If you want an even number, choose 13 or 14 (13/14)", "14", Difficulty::Stretch),
        ],
        Grade::G4 => vec![
            q("To get a multiple of 5, choose 18 or 20 (18/20)", "20", Difficulty::Core),
            q("To get a number divisible by 3, choose 14 or 15 (14/15)", "15", Difficulty::Core),
            q("Stretch: Best choice to keep it under 50: 49 or 52 (49/52)", "49", Difficulty::Stretch),
        ],
        Grade::G5 => vec![
            q("To make 1 whole, choose 1/2 + 1/2 OR 1/3 + 1/3 (write 1/2+1/2 or 1/3+1/3)", "1/2+1/2", Difficulty::Core),
            q("Best choice: Which is larger? 0.6 or 0.56 (0.6/0.56)", "0.6", Difficulty::Core),
            q("Stretch: Which is larger? 3/4 or 2/3 (3/4 or 2/3)", "3/4", Difficulty::Stretch),
        ],
    }
}

// Small helper to keep question construction tidy.
fn q(prompt: &'static str, answer: &'static str, difficulty: Difficulty) -> Question {
    Question { prompt, answer, difficulty }
}