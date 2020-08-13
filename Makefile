##
# Project Title
#
# Copyright (c) 2019-2020 John Goerzen

#     This program is free software: you can redistribute it and/or modify
#     it under the terms of the GNU General Public License as published by
#     the Free Software Foundation, either version 3 of the License, or
#     (at your option) any later version.

#     This program is distributed in the hope that it will be useful,
#     but WITHOUT ANY WARRANTY; without even the implied warranty of
#     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#     GNU General Public License for more details.

#     You should have received a copy of the GNU General Public License
#     along with this program.  If not, see <http://www.gnu.org/licenses/>.

.PHONY: ghp-fix build deploy

COVID19DB_PATH ?= covid19.db

build:
	if [ ! -e $(COVID19DB_PATH) ]; then \
		curl -L -o covid19db.zip https://github.com/jgoerzen/covid19db/releases/download/v0.1.0/covid19db.zip && \
        unzip covid19db.zip && rm covid19db.zip; fi
	cargo run $(COVID19DB_PATH)
	cat static/header.html html-fragments/all.html static/footer.html > html-entire/full.html

ghp-fix:
	sed -i 's/^ *//g' static/*.html html-fragments/*.html

deploy: build ghp-fix
	cp -r website deploy
	mkdir deploy/graphs
	cp static/script.html deploy/graphs/
	cp html-fragments/* deploy/graphs/
	TZ=America/Chicago date > deploy/graphs/timestamp

# end
