use rustify::artist::ArtistData;

use dotenv::from_path;

use std::env::var;
use std::time::Instant;

#[test]
fn all() {
    let path = r#"C:\Users\conno\rustify\tests\.env"#;
    from_path(path).unwrap();

    let id: String = var("USER_ID").unwrap();
    let secret: String = var("USER_SECRET").unwrap();

    let start = Instant::now();

    let mut result = ArtistData::new(id, secret);

    //Single artists:
    //println!("{:?}", result.get_artist("0s1ec6aPpRZ4DCj15w1EFg").run());
    //println!("{:?}", result.get_artist("0C0XlULifJtAgn6ZNCW2eu").run());
    println!("{:?}", result.get_artist("0s1ec6aPpRZ4DCj15w1EFg").run().name());

    //2+ artists:
    println!("{:?}", result.get_artists(&["0s1ec6aPpRZ4DCj15w1EFg", "0C0XlULifJtAgn6ZNCW2eu"]).run().names());

    //Albums
    //println!("{:?}", result.get_albums("0C0XlULifJtAgn6ZNCW2eu", None, None, Some(1), None).run());

    //Top Tracks
    //println!("{:?}", result.get_top_tracks("0C0XlULifJtAgn6ZNCW2eu", "US").run());

    //Related Artists
    //println!("{:?}", result.get_related_artists("0C0XlULifJtAgn6ZNCW2eu").run());

    println!("{:?}", start.elapsed());
}