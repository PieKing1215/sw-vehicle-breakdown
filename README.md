# SW Vehicle Breakdown

https://pieking1215.github.io/sw-vehicle-breakdown/

Web tool for creating cost/mass breakdowns for Vehicles from [Stormworks: Build and Rescue](https://store.steampowered.com/app/573090/Stormworks_Build_and_Rescue/).

You can drag and drop or select a vehicle XML file, and it'll generate a table of the count, cost, and mass of every component used in your vehicle.<br/>
Once it's loaded you can click the column titles to sort by that column.<br/>
*(this is all calculated in your browser, there's no uploading or server or anything)*

![image](https://github.com/user-attachments/assets/ae6562a3-ba8c-4fe5-a33d-1c2a2a2d9adb)


## Interesting Technical Details

This project is built using [Perseus](https://github.com/framesurge/perseus), a web framework for Rust.<br/>
The Rust code runs at two phases:

At compile time, data about the game's components (ie. the Rom) is parsed directly from the game's rom folder and included when it generates the static webpage and webassembly code.<br/>
An automated [GitHub Actions workflow](https://github.com/PieKing1215/sw-vehicle-breakdown/actions/workflows/autobuild.yml) runs every Wednesday night that updates the game, recompiles the rom data, and redeploys the page to GitHub pages.<br/>
This means if a game update adds/changes components, the site will automatically update without any manual work needed (unless they do something very unusual)

At runtime (when you open the page in your browser), the Rust/wasm code loads the rom data and handles all of the XML parsing and statistics functionality locally.<br/>
This means there's no uploading step or server needed which means GitHub pages can host it for free :)

## Licenses

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Disclaimer
I am not personally affiliated with Stormworks: Build and Rescue or Geometa, nor has Stormworks: Build and Rescue or Geometa endorsed this product.<br/>
Stormworks: Build and Rescue and any of its content or materials are the property of their respective owners.
