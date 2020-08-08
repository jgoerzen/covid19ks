# COVID-19 in Kansas: Masks vs. No-Masks

In 2020, some counties in Kansas adopted a mask requirement while others didn't.  The Kansas Department of Health and Environment published a [chart](kdhe-chart.pdf) illustrating COVID-19 changes in masks vs. without counties.

Some people questioned whether this chart was misleading due to its use of different Y-axis.  This repository produces a similar chart with a unified Y-axis and shows that the result was not misleading.  Here is the chart I generated:

![](main.png)

The source data is us-counties.csv from the [covid-19-data set](https://github.com/nytimes/covid-19-data).  It is passed as the one parameter to the executable in this repository.  [This particular version](https://github.com/nytimes/covid-19-data/blob/42590181052a7591385562a59fdd545bd478f763/us-counties.csv) was used for the generation of the graph here, so you can verify these results for yourself.

# Copyright

Copyright (c) 2019-2020 John Goerzen

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

