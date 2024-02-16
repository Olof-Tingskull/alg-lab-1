mod coins;
mod plot;
mod winning_streak;
mod arguments;
mod track_time;
mod assignments;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    assignments::assignment_1a()?;
    assignments::assignment_1b()?;
    assignments::assignment_1c()?;
    assignments::assignment_1d()?;
    assignments::assignment_1e()?;
    assignments::assignment_2a()?;
    assignments::assignment_2b()?;
    assignments::assignment_2c()?;
    assignments::assignment_2d()?;

    Ok(())
}
