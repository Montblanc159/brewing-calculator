#!/usr/bin/env bash
# Updates jsons converted from brewdb sqlite file with a rusty jsonifier tool
set -eux

git submodule update --recursive --remote --init

curl -o brewdb-jsonifier -L https://github.com/Montblanc159/brewdb-jsonifier/releases/latest/download/brewdb-jsonifier-x86_64-unknown-linux-gnu

chmod +x brewdb-jsonifier

./brewdb-jsonifier

mv hops.json malts.json yeasts.json ./src/app/modules/ingredients_index/assets/