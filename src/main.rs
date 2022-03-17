// This file is part of senpy-cli <https://github.com/senpy-club/cli>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

mod export;

use clap::{Arg, Command};

use crate::export::Exporter;

fn main() {
  let matches = clap::Command::new(env!("CARGO_PKG_NAME"))
    .version(&*format!(
      "{}(1)-{}-({})-{}",
      env!("CARGO_PKG_VERSION"),
      env!("VERGEN_CARGO_PROFILE"),
      env!("VERGEN_CARGO_TARGET_TRIPLE"),
      env!("VERGEN_GIT_SHA").get(0..7).unwrap(),
    ))
    .author(env!("CARGO_PKG_AUTHORS"))
    .subcommand_required(true)
    .subcommands(vec![
      Command::new("languages")
        .alias("langs")
        .about("Prints all available languages")
        .arg(Arg::new("time").long("time").short('t'))
        .arg(Arg::new("file").long("file").short('f').takes_value(true)),
      Command::new("language")
        .alias("lang")
        .arg_required_else_help(true)
        .arg(Arg::new("language").required(true))
        .about("Prints all available images of a given language")
        .arg(Arg::new("time").long("time").short('t'))
        .arg(Arg::new("file").long("file").short('f').takes_value(true)),
      Command::new("random")
        .alias("rand")
        .about("Prints a random language and image pair")
        .arg(Arg::new("time").long("time").short('t'))
        .arg(Arg::new("file").long("file").short('f').takes_value(true)),
    ])
    .get_matches();

  match matches.subcommand() {
    Some(("languages", matches_languages)) => {
      let time = std::time::Instant::now();
      let languages = senpy::languages();
      let time_taken = time.elapsed();

      match languages {
        Ok(languages) => {
          let time_export = std::time::Instant::now();
          let mut exported = false;

          if matches_languages.is_present("file") {
            if matches_languages.value_of("file").unwrap().contains("json") {
              crate::export::JsonExporter::vec_to_file(
                matches_languages.value_of("file").unwrap(),
                languages,
              );

              exported = true;
            } else if matches_languages
              .value_of("file")
              .unwrap()
              .contains("yaml")
              || matches_languages.value_of("file").unwrap().contains("yml")
            {
              crate::export::YamlExporter::vec_to_file(
                matches_languages.value_of("file").unwrap(),
                languages,
              );

              exported = true;
            } else if matches_languages
              .value_of("file")
              .unwrap()
              .contains("dhall")
            {
              crate::export::DhallExporter::vec_to_file(
                matches_languages.value_of("file").unwrap(),
                languages,
              );

              exported = true;
            }

            if exported {
              println!(
                "time taken to export to file: {} ns",
                time_export.elapsed().as_nanos(),
              );
            }
          } else {
            // https://stackoverflow.com/a/30380640/14452787
            println!(
              "{0: <10} {1: <10}",
              "Languages",
              if matches_languages.is_present("time") {
                "Time"
              } else {
                ""
              }
            );

            for (index, language) in languages.into_iter().enumerate() {
              println!(
                "{0: <10} {1: <10}",
                language,
                if matches_languages.is_present("time") && index == 0 {
                  time_taken.as_millis().to_string()
                } else {
                  "".to_string()
                }
              );
            }
          }
        }
        Err(e) => println!("{0: <10} {1: <10}", "Error", e),
      }
    }
    Some(("language", matches_language)) => {
      let time = std::time::Instant::now();
      let languages = senpy::languages();
      let time_taken = time.elapsed();

      match languages {
        Ok(languages) => {
          let language = &languages
            .into_iter()
            .filter(|l| {
              l.to_lowercase()
                == matches_language
                  .value_of("language")
                  .unwrap()
                  .to_lowercase()
            })
            .collect::<Vec<String>>()[0];
          let time_2 = std::time::Instant::now();
          let images = senpy::language(language.to_string().as_str());
          let time_taken_2 = time_2.elapsed();

          match images {
            Ok(images) => {
              let time_export = std::time::Instant::now();
              let mut exported = false;

              if matches_language.is_present("file") {
                if matches_language.value_of("file").unwrap().contains("json") {
                  crate::export::JsonExporter::vec_to_file(
                    matches_language.value_of("file").unwrap(),
                    images,
                  );

                  exported = true;
                } else if matches_language
                  .value_of("file")
                  .unwrap()
                  .contains("yaml")
                  || matches_language.value_of("file").unwrap().contains("yml")
                {
                  crate::export::YamlExporter::vec_to_file(
                    matches_language.value_of("file").unwrap(),
                    images,
                  );

                  exported = true;
                } else if matches_language
                  .value_of("file")
                  .unwrap()
                  .contains("dhall")
                {
                  crate::export::DhallExporter::vec_to_file(
                    matches_language.value_of("file").unwrap(),
                    images,
                  );

                  exported = true;
                }

                if exported {
                  println!(
                    "time taken to export to file: {} ns",
                    time_export.elapsed().as_nanos(),
                  );
                }
              } else {
                // https://stackoverflow.com/a/30380640/14452787
                println!(
                  "{0: <10} {1: <10}",
                  "Images",
                  if matches_language.is_present("time") {
                    "Time"
                  } else {
                    ""
                  }
                );

                for (index, image) in images.into_iter().enumerate() {
                  println!(
                    "{0: <10} {1: <10}",
                    image,
                    if matches_language.is_present("time") && index == 0 {
                      (time_taken + time_taken_2).as_millis().to_string()
                    } else {
                      "".to_string()
                    }
                  );
                }
              }
            }
            Err(e) => println!("{0: <10} {1: <10}", "Error", e),
          }
        }
        Err(e) => println!("{0: <10} {1: <10}", "Error", e),
      }
    }
    Some(("random", matches_random)) => {
      let time = std::time::Instant::now();
      let random = senpy::random();
      let time_taken = time.elapsed();

      match random {
        Ok(image) => {
          let time_export = std::time::Instant::now();
          let mut exported = false;

          if matches_random.is_present("file") {
            if matches_random.value_of("file").unwrap().contains("json") {
              crate::export::JsonExporter::random_to_file(
                matches_random.value_of("file").unwrap(),
                image,
              );

              exported = true;
            } else if matches_random.value_of("file").unwrap().contains("yaml")
              || matches_random.value_of("file").unwrap().contains("yml")
            {
              crate::export::YamlExporter::random_to_file(
                matches_random.value_of("file").unwrap(),
                image,
              );

              exported = true;
            } else if matches_random.value_of("file").unwrap().contains("dhall")
            {
              crate::export::DhallExporter::random_to_file(
                matches_random.value_of("file").unwrap(),
                image,
              );

              exported = true;
            }

            if exported {
              println!(
                "time taken to export to file: {} ns",
                time_export.elapsed().as_nanos(),
              );
            }
          } else {
            // https://stackoverflow.com/a/30380640/14452787
            println!(
              "{0: <10} {1: <10} {2: <10}",
              "Language",
              "Image",
              if matches_random.is_present("time") {
                "Time"
              } else {
                ""
              }
            );
            println!(
              "{0: <10} {1: <10} {2: <10}",
              image.language,
              image.image,
              if matches_random.is_present("time") {
                time_taken.as_millis().to_string()
              } else {
                "".to_string()
              }
            );
          }
        }
        Err(e) => println!("{0: <10} {1: <10}", "Error", e),
      }
    }
    _ => unreachable!(),
  }
}
