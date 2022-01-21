# Sabio-rk example

Example parsing sabio-rk SBML files. 

## Steps to reproduce

First, clone the repository and make sure that [jupyter lab](https://jupyter.org/) and [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) are installed.

```bash
git clone https://github.com/computer-aided-biotech/sabio_stuff.git
cd sabio_stuff
```

1. Retrieval of information. This is the SBML corresponding to the ethanol dehydrogenase (`SabioReactionID` 597) (will generate a file `sabio.xml`).
  ```bash
  curl -L http://sabiork.h-its.org/sabioRestWebServices/searchKineticLaws/sbml\?q\=SabioReactionID:"597" > sabio597.xml
  ```
2. Parsing of the SBML doc into a TSV table (will generate a file `sabio.tsv`).
  ```bash
  cargo build --release
  ./target/release/sabio_stuff sabio.xml > sabio.tsv
  ```
3. Plotting the results. Open the file `./notebooks/sabio_587.ipynb` in jupyter lab.
