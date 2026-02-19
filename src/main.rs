use chrono::{NaiveDate, Utc, Duration};

fn main() {
 
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <YYYY-MM-DD>", args[0]);
        std::process::exit(1);
    }

    let date_str = &args[1];
    let birth_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .expect("Invalid date format. Use YYYY-MM-DD");

    println!("Parsed Birthdate: {}", birth_date);

    let today_date:NaiveDate = Utc::now().date_naive();

    let diff = today_date.signed_duration_since(birth_date);
    let days_alive = diff.num_days(); 
    
    println!("Today's date: {}", today_date);
    println!("Days between: {}", days_alive);
    println!("-------------------------------------Plotting Biorhythms-------------------------------------");

    let mut days: i64 = 0;

    while days < 30 {
        let plot_date = today_date.checked_add_signed(Duration::days(days)).unwrap();
        let mut plot_string = plot_date.format("%Y-%m-%d").to_string();
        plot_string.push_str(": ");
        let phy: i32 = (f64::sin(2.0 * std::f64::consts::PI * (days_alive as f64 + days as f64) / 23.0) * 30.0) as i32;   
        let emo: i32 = (f64::sin(2.0 * std::f64::consts::PI * (days_alive as f64 + days as f64) / 28.0) * 30.0) as i32;   
        let int: i32 = (f64::sin(2.0 * std::f64::consts::PI * (days_alive as f64 + days as f64) / 33.0) * 30.0) as i32;   
        for j in -40..40 {
            match j {
                x if x == phy => {plot_string.push('P')}
                x if x == emo => {plot_string.push('E')}
                x if x == int => {plot_string.push('I')}
                _ => plot_string.push(' '),
            }
        }
        println!("{}: ",plot_string);
        days +=1;
 
    }
}
