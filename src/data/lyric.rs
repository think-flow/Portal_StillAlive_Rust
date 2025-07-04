use std::sync::LazyLock;

pub static LYRICS: LazyLock<[Lyric; 121]> = std::sync::LazyLock::new(|| {
    [
        // Page 1
        Lyric::new("Forms FORM-29827281-12:", 0, -1.0, 0),
        Lyric::new("Test Assessment Report", 200, -1.0, 0),
        Lyric::new("\0\0\0\0\0\0\0", 400, -1.0, 0), // Keep flushing the buffer
        Lyric::new("", 710, 0.0, 4),                // Music start
        Lyric::new("This was a triumph.", 730, 2.0, 0),
        Lyric::new("", 930, 0.0, 5), // Credits start
        Lyric::new("I'm making a note here:", 1123, 2.0, 0),
        Lyric::new("HUGE SUCCESS.", 1347, 1.7, 0),
        Lyric::new("It's hard to overstate", 1627, -1.0, 0),
        Lyric::new("my satisfaction.", 1873, 2.6, 0),
        Lyric::new("Aperture Science", 2350, 1.8, 0),
        Lyric::new(0, 2350, 0.0, 2), // ASCII 1
        Lyric::new("We do what we must", 2733, 1.6, 0),
        Lyric::new("because we can.", 2910, 1.5, 0),
        Lyric::new("For the good of all of us.", 3237, -1.0, 0),
        Lyric::new(1, 3500, 0.0, 2), // ASCII 2
        Lyric::new("Except the ones who are dead.", 3567, -1.0, 0),
        Lyric::new("", 3717, 0.05, 0),
        Lyric::new(0, 3717, 0.0, 2), // ASCII 1
        Lyric::new("But there's no sense crying", 3787, -1.0, 0),
        Lyric::new("over every mistake.", 3973, 1.77, 0),
        Lyric::new("You just keep on trying", 4170, -1.0, 0),
        Lyric::new("till you run out of cake.", 4370, -1.0, 0),
        Lyric::new(2, 4500, 0.0, 2), // ASCII 3
        Lyric::new("And the Science gets done.", 4570, -1.0, 0),
        Lyric::new("And you make a neat gun.", 4767, -1.0, 0),
        Lyric::new(0, 4903, 0.0, 2), // ASCII 1
        Lyric::new("For the people who are", 4973, -1.0, 0),
        Lyric::new("still alive.", 5110, 1.6, 1),
        // Page 2
        Lyric::new(0, 5353, 0.0, 3), // Clear lyrics
        Lyric::new("Forms FORM-55551-5:", 5413, -1.0, 0),
        Lyric::new("Personnel File Addendum:", 5477, 1.13, 0),
        Lyric::new("", 5650, 0.05, 0),
        Lyric::new("Dear <<Subject Name Here>>,", 5650, -1.0, 0),
        Lyric::new("", 5900, -1.0, 0),
        Lyric::new("I'm not even angry.", 5900, 1.86, 0),
        Lyric::new("I'm being ", 6320, -1.0, 1),
        Lyric::new("so ", 6413, -1.0, 1),
        Lyric::new("sincere right now.", 6470, 1.9, 0),
        Lyric::new("Even though you broke ", 6827, -1.0, 1),
        Lyric::new(3, 7020, 0.0, 2), // ASCII 4
        Lyric::new("my heart.", 7090, -1.0, 0),
        Lyric::new("And killed me.", 7170, 1.43, 0),
        Lyric::new(4, 7300, 0.0, 2), // ASCII 5
        Lyric::new("And tore me to pieces.", 7500, 1.83, 0),
        Lyric::new("And threw every piece ", 7900, -1.0, 1),
        Lyric::new("into a fire.", 8080, 1.8, 0),
        Lyric::new(5, 8080, 0.0, 2), // ASCII 6
        Lyric::new("As they burned it hurt because", 8430, -1.0, 0),
        Lyric::new(6, 8690, 0.0, 2), // ASCII 7
        Lyric::new("I was so happy for you!", 8760, 1.67, 0),
        Lyric::new("Now, these points of data", 8960, -1.0, 0),
        Lyric::new("make a beautiful line.", 9167, -1.0, 0),
        Lyric::new("And we're out of beta.", 9357, -1.0, 0),
        Lyric::new("We're releasing on time.", 9560, -1.0, 0),
        Lyric::new(4, 9700, 0.0, 2), // ASCII 5
        Lyric::new("So I'm GLaD I got burned.", 9770, -1.0, 0),
        Lyric::new(2, 9913, 0.0, 2), // ASCII 3
        Lyric::new("Think of all the things we learned", 9983, -1.0, 0),
        Lyric::new(0, 10120, 0.0, 2), // ASCII 1
        Lyric::new("For the people who are", 10190, -1.0, 0),
        Lyric::new("Still alive.", 10327, 1.8, 0),
        // Page 3
        Lyric::new(0, 10603, 0.0, 3), // Clear lyrics
        Lyric::new("Forms FORM-55551-6:", 10663, -1.0, 0),
        Lyric::new("Personnel File Addendum Addendum:", 10710, 1.36, 0),
        Lyric::new("", 10710, 0.05, 0),
        Lyric::new("One last thing:", 10910, -1.0, 0),
        Lyric::new("", 11130, 0.05, 0),
        Lyric::new("Go ahead and leave ", 11130, -1.0, 1),
        Lyric::new("me.", 11280, 0.5, 0),
        Lyric::new("I think I'd prefer to stay ", 11507, -1.0, 1),
        Lyric::new("inside.", 11787, 1.13, 0),
        Lyric::new("Maybe you'll find someone else", 12037, -1.0, 0),
        Lyric::new("To help you.", 12390, 1.23, 0),
        Lyric::new("Maybe Black ", 12737, -1.0, 1),
        Lyric::new(7, 12787, 0.0, 2), // ASCII 8
        Lyric::new("Mesa...", 12857, 2.7, 0),
        Lyric::new("THAT WAS A JOKE.", 13137, 1.46, 1),
        Lyric::new(" FAT CHANCE.", 13387, 1.1, 0),
        Lyric::new("Anyway, ", 13620, -1.0, 1),
        Lyric::new(8, 13670, 0.0, 2), // ASCII 9
        Lyric::new("this cake is great.", 13740, -1.0, 0),
        Lyric::new("It's so delicious and moist.", 13963, -1.0, 0),
        Lyric::new(9, 14123, 0.0, 2), // ASCII 10
        Lyric::new("Look at me still talking", 14193, -1.0, 0),
        Lyric::new(1, 14320, 0.0, 2), // ASCII 2
        Lyric::new("when there's science to do.", 14390, -1.0, 0),
        Lyric::new(0, 14527, 0.0, 2), // ASCII 1
        Lyric::new("When I look out there,", 14597, -1.0, 0),
        Lyric::new("It makes me GLaD I'm not you.", 14767, -1.0, 0),
        Lyric::new(2, 14913, 0.0, 2), // ASCII 3
        Lyric::new("I've experiments to run.", 14983, -1.0, 0),
        Lyric::new(4, 15120, 0.0, 2), // ASCII 5
        Lyric::new("There is research to be done.", 15190, -1.0, 0),
        Lyric::new(0, 15320, 0.0, 2), // ASCII 1
        Lyric::new("On the people who are", 15390, -1.0, 0),
        Lyric::new("still alive", 15553, 2.0, 1),
        // Page 4
        Lyric::new(0, 15697, 0.0, 3), // Clear lyrics
        Lyric::new("", 15757, 0.05, 0),
        Lyric::new("", 15757, 0.05, 0),
        Lyric::new("", 15757, 0.05, 0),
        Lyric::new("PS: And believe me I am", 15757, -1.0, 0),
        Lyric::new("still alive.", 15960, 1.13, 0),
        Lyric::new("PPS: I'm doing Science and I'm", 16150, -1.0, 0),
        Lyric::new("still alive.", 16363, 1.13, 0),
        Lyric::new("PPPS: I feel FANTASTIC and I'm", 16550, -1.0, 0),
        Lyric::new("still alive.", 16760, -1.0, 0),
        Lyric::new("", 16860, -1.0, 0),
        Lyric::new("FINAL THOUGH:", 16860, -1.0, 0),
        Lyric::new("While you're dying I'll be", 16993, -1.0, 0),
        Lyric::new("still alive.", 17157, -1.0, 0),
        Lyric::new("", 17277, -1.0, 0),
        Lyric::new("FINAL THOUGH PS:", 17277, -1.0, 0),
        Lyric::new("And when you're dead I will be", 17367, -1.0, 0),
        Lyric::new("still alive.", 17550, 1.13, 0),
        Lyric::new("", 17550, -1.0, 0),
        Lyric::new("", 17550, 0.05, 0),
        Lyric::new("STILL ALIVE", 17760, 1.13, 0),
        Lyric::new(0, 17900, 0.0, 3), // Clear lyrics
        Lyric::new(0, 18500, 0.0, 3), // Clear lyrics
        Lyric::new("ENDENDENDENDENDENDENDEND", 18500, 0.05, 9),
    ]
});

#[derive(Debug)]
pub struct Lyric {
    pub words: WordsContent,
    pub time: f64,
    pub interval: f64,
    pub mode: i32,
}

#[derive(Debug)]
pub enum WordsContent {
    Str(&'static str),
    Int(i32),
}

impl From<&'static str> for WordsContent {
    fn from(value: &'static str) -> Self {
        WordsContent::Str(value)
    }
}

impl From<i32> for WordsContent {
    fn from(value: i32) -> Self {
        WordsContent::Int(value)
    }
}

impl Lyric {
    fn new(words: impl Into<WordsContent>, time: i32, interval: f64, mode: i32) -> Self {
        Lyric {
            words: words.into(),
            time: time as f64,
            interval,
            mode,
        }
    }
}
