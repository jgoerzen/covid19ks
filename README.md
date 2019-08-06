# CDC death data visualization

This is used to create Sankey diagrams of data of causes of death as obtained from the [CDC Wonder Data Request](https://wonder.cdc.gov/controller/datarequest/D76) system.  It is a Rust program that collates and filters the data to make it presentable.  The parameters to the coalescepct function in main.rs are particularly relevant for filtering.

I have included an example png of the output in this directory.  It is generated as follows:

The data comes from [CDC Wonder Data Request](https://wonder.cdc.gov/controller/datarequest/D76).  Parameters are as follows: ten-year age groups: 35-44 and 45-54; year: 2016; Group by: ICD chapter, then ICD sub-chapter, then cause of death, with "export results" checked.

The output from this program is then sent to [Sankeymatic Build](http://sankeymatic.com/build/).

This particular age group was selected because it tends to exclude disproportionate causes of death at the very young or very old edge of the spectrum; "old age" is not a listed cause of death.  This, therefore, tends to show "what kills people early."

