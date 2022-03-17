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

use std::{
  fs::File,
  io::{BufWriter, Write},
};

use senpy::Random;

pub trait Exporter {
  fn random_to_file(_: &str, _: Random);
  fn vec_to_file(_: &str, _: Vec<String>);
}

pub struct JsonExporter;
pub struct YamlExporter;
pub struct DhallExporter;

impl Exporter for JsonExporter {
  fn random_to_file(filename: &str, random: Random) {
    let writer = BufWriter::new(File::create(filename).unwrap());

    serde_json::to_writer_pretty(
      writer,
      &serde_json::json!({
        "language": random.language,
        "image": random.image,
      }),
    )
    .unwrap();
  }

  fn vec_to_file(filename: &str, languages: Vec<String>) {
    let writer = BufWriter::new(File::create(filename).unwrap());

    serde_json::to_writer_pretty(writer, &languages).unwrap();
  }
}

impl Exporter for YamlExporter {
  fn random_to_file(filename: &str, random: Random) {
    let writer = BufWriter::new(File::create(filename).unwrap());

    serde_yaml::to_writer(writer, &random).unwrap()
  }

  fn vec_to_file(filename: &str, languages: Vec<String>) {
    let writer = BufWriter::new(File::create(filename).unwrap());

    serde_yaml::to_writer(writer, &languages).unwrap();
  }
}

impl Exporter for DhallExporter {
  fn random_to_file(filename: &str, random: Random) {
    let mut writer = BufWriter::new(File::create(filename).unwrap());

    writer
      .write_all(
        serde_dhall::serialize(&random)
          .to_string()
          .unwrap()
          .as_bytes(),
      )
      .unwrap();
  }

  fn vec_to_file(filename: &str, languages: Vec<String>) {
    let mut writer = BufWriter::new(File::create(filename).unwrap());

    writer
      .write_all(
        serde_dhall::serialize(&languages)
          .to_string()
          .unwrap()
          .as_bytes(),
      )
      .unwrap();
  }
}
