pub fn build_proverb(pieces: Vec<&str>) -> String {
    let fmt_stanza = |piece_pair: (&&str, &&str)| -> String {
        let (current_piece, next_piece) = piece_pair;
        format!("For want of a {} the {} was lost.", current_piece, next_piece)
    };

    let fmt_last_stanza = |num_pieces: usize| -> String {
        match num_pieces {
            0       => "".to_string(),
            1 ... 2 => "And all for the want of a nail.".to_string(),
            _       => "And all for the want of a horseshoe nail.".to_string()
        }
    };

    let mut stanzas =
        pieces
            .iter()
            .zip(pieces.iter().skip(1))
            .map(fmt_stanza)
            .collect::<Vec<String>>();

    stanzas.push(fmt_last_stanza(pieces.len()));
    stanzas.join("\n")
}
