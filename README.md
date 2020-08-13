# COVID-19 in Kansas

![build](https://github.com/jgoerzen/covid19ks/workflows/build/badge.svg) 

This repository contains software that automatically generates graphs about COVID-19 in Kansas. 

**[Click here to see the current graphs](https://jgoerzen.github.io/covid19ks/)**

# Background

This project started because, in 2020, some counties in Kansas adopted a mask requirement while others didn't.  The Kansas Department of Health and Environment published a [chart](kdhe-chart.pdf) illustrating COVID-19 changes in masks vs. without counties.

Some people questioned whether this chart was misleading due to its use of different Y-axis.  This repository produces a similar chart with a unified Y-axis and shows that the result was not misleading.  Here is the chart I generated using an earlier version of this software:

![](main.png)

I have [many more graphs](https://jgoerzen.github.io/covid19ks/) also available.

# Invocation

This software uses the [covid19db](https://github.com/jgoerzen/covid19db) to generate the graphs.  You need to download (or generate) the covid19.db file.  The easiest option is to just donwload the ZIP to the current directory, unzip it, and run it from there.

``` sh
cd covid19ks
curl -L -o covid19db.zip https://github.com/jgoerzen/covid19db/releases/download/v0.1.0/covid19db.zip
unzip covid19db.zip
cargo run --release
```

With these commands, you can verify these results for yourself.  If you don't already have Rust installed, see the [Rust installation](https://www.rust-lang.org/tools/install) page.

# Copyright

    This code is Copyright (c) 2019-2020 John Goerzen

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.


This repository contains only tools for obtaining data and no data itself, though the data itself may be available elsewhere on Github.  If you use the data accumulated by this program, or download it, you may be required to acknowledge the source.  Here are some details:

## cdataset - New York Times

In general, we are making this data publicly available for broad, noncommercial public use including by medical and public health researchers, policymakers, analysts and local news media.

If you use this data, you must attribute it to “The New York Times” in any publication. If you would like a more expanded description of the data, you could say “Data from The New York Times, based on reports from state and local health agencies.”

If you use it in an online presentation, we would appreciate it if you would link to our U.S. tracking page at https://www.nytimes.com/interactive/2020/us/coronavirus-us-cases.html.

If you use this data, please let us know at covid-data@nytimes.com.

See our LICENSE for the full terms of use for this data.

This license is co-extensive with the Creative Commons Attribution-NonCommercial 4.0 International license, and licensees should refer to that license (CC BY-NC) if they have questions about the scope of the license.

[source](https://github.com/nytimes/covid-19-data)

## cdataset and loc_lookup - Johns Hopkins

1.    This data set is licensed under the Creative Commons Attribution 4.0 International (CC BY 4.0) by the Johns Hopkins University on behalf of its Center for Systems Science in Engineering. Copyright Johns Hopkins University 2020.
2.    Attribute the data as the "COVID-19 Data Repository by the Center for Systems Science and Engineering (CSSE) at Johns Hopkins University" or "JHU CSSE COVID-19 Data" for short, and the url: https://github.com/CSSEGISandData/COVID-19.
3.    For publications that use the data, please cite the following publication: "Dong E, Du H, Gardner L. An interactive web-based dashboard to track COVID-19 in real time. Lancet Inf Dis. 20(5):533-534. doi: 10.1016/S1473-3099(20)30120-1"

[source](https://github.com/CSSEGISandData/COVID-19)

## rtlive - rt.live

We just ask that you cite Rt.live as the source and link where appropriate.

[source](https://rt.live/faq)

## covid19tracking - COVID-19 Tracking Project

You are welcome to copy, distribute, and develop data and website content from The COVID Tracking Project at The Atlantic for all healthcare, medical, journalistic and non-commercial uses, including any personal, editorial, academic, or research purposes.

The COVID Tracking Project at The Atlantic data and website content is published under a Creative Commons CC BY-NC-4.0 license, which requires users to attribute the source and license type (CC BY-NC-4.0) when sharing our data or website content. The COVID Tracking Project at The Atlantic also grants permission for any derivative use of this data and website content that supports healthcare or medical research (including institutional use by public health and for-profit organizations), or journalistic usage (by nonprofit or for-profit organizations). All other commercial uses are not permitted under the Creative Commons license, and will require permission from The COVID Tracking Project at The Atlantic.

[source](https://covidtracking.com/about-data/license)
