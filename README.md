This is my first attempt at a small RUST application.

I struggled with 2 things. 

First Date types and conversions.

Second, the match control flow expression. 
My instincts kept telling me that I should be able to write

        for j in -40..40 {
            match j {
                phy => {plot_string.push('P')}
                emo => {plot_string.push('E')}
                int => {plot_string.push('I')}
                _ => plot_string.push(' '),
            }
        }

and quite honestly, I still am not positive why it doesn't? Is it 
because of Ownership?

