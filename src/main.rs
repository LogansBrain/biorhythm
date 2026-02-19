// include NaiveDate, Uta, and Duration from the chorno crate.
use chrono::{NaiveDate, Utc, Duration};

fn main() {
 
    // assigns command line arguments to variable args as a vector of strings from the std::env::args function
    // through the collect method.
    let args: Vec<String> = std::env::args().collect();

    // if there are not 2 arguments
    if args.len() != 2 {
        // print an error message to the error console.
        eprintln!("Usage: {} <YYYY-MM-DD>", args[0]);
        // and exit the program with an error code.
        std::process::exit(1);
    }

    // assign the first argument after the program name to date_str
    let date_str = &args[1];

    // convert the date_str argument to a NaiveDate type and hold it in birth_date
    let birth_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        // if the argument does not conform to the expected date format show an error
        .expect("Invalid date format. Use YYYY-MM-DD");

    // This prints the converted, parsed argument birth_date for user feedback / visual debugging
    println!("Parsed Birthdate: {}", birth_date);

    // retrieve the current UTC date, convert it to NaiveDate format and store it in variable today_date
    let today_date:NaiveDate = Utc::now().date_naive();

    // assign a variable diff with the difference between today_date and birth_date using signed_duration_since
    let diff = today_date.signed_duration_since(birth_date);

    // calculate and store the number of days from the diff (duration) object in days_alive variable
    let days_alive = diff.num_days(); 
    
    // Print todays date
    println!("Today's date: {}", today_date);
    // Print the calculated value of days since birth_date
    println!("Days between: {}", days_alive);
    // Make the output a little prettier
    println!("-------------------------------------Plotting Biorhythms-------------------------------------");

    // days is a counter for the number of iterations to plot
    let mut days: i64 = 0;

    // create a while loop which should resolve to (0..29)
    while days < 30 {

        // plot_date represents the day being plotted
        // It is today_date + the number of days represented by our loop
        let plot_date = today_date.checked_add_signed(Duration::days(days)).unwrap();

        // create a mutable string called plot_string which starts with the plot_date
        // this string is a line which will contain the plot points represented by characters
        let mut plot_string = plot_date.format("%Y-%m-%d").to_string();

        // add a ":" and a space to the plot_string to aid output readability
        plot_string.push_str(": ");

        // create 3 plot points, phy, emo, int using sine wave function sin() and PI constants
        // each point is calculated using a different cycle and cast as a 32bit integer.
        // plot_point = sin(2*pi*days_alive+days (loop) / "daily cycle 23/28/33") then multiply by 30 for graph scale
        let phy: i32 = (f64::sin(2.0 * std::f64::consts::PI * (days_alive as f64 + days as f64) / 23.0) * 30.0) as i32;   
        let emo: i32 = (f64::sin(2.0 * std::f64::consts::PI * (days_alive as f64 + days as f64) / 28.0) * 30.0) as i32;   
        let int: i32 = (f64::sin(2.0 * std::f64::consts::PI * (days_alive as f64 + days as f64) / 33.0) * 30.0) as i32;   

        // j represents the current position in the graph from -40 to 40
        for j in -40..40 {

            // if that position in the graph coorelates to a graphable point then add a character representing that cycle        
            // *authors note: I was trying to wrap my head around this match expression so I force myself to use it.
            // It makes no allowance for the graph to show points that might intersect.
            // So if any of the values share a common point only the first evaluated one will show on the graph
            // if I feel like mashing my head against the desk for a few hours I might come back and create a more
            // complex match guard which would plot an 'X' where 2 or more cycles overlap.
            match j {
                x if x == phy => {plot_string.push('P')}
                x if x == emo => {plot_string.push('E')}
                x if x == int => {plot_string.push('I')}

                // if there is no point to plot, add a space to the plot_string
                _ => plot_string.push(' '),
            }
        }
        // print the string which is a text representation of the 3 sine waves.
        println!("{}: ",plot_string);

        // increment the day to graph and repeat
        days +=1;
 
    }
}
