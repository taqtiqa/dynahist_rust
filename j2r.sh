#!/usr/bin/env bash
#
# Run the edit item unit tests (Robot Framework).
#
# USAGE:
#     git clone git@bitbucket.org:horizonx/data-preprocessing.git
#     cd data-preprocessing
#     ./src/scripts/run-edit-item-tests.sh
#
# REQUIREMENTS:
#   - Python 3.6.9
#   - Robot Framework 4.1
#   - find (GNU findutils) 4.7.0
#   - sort (GNU coreutils) 8.28
#
# The test files are:
#   - ${repo_root}/features/items/edit/*.robot
#
# The log file is:
#   - ${repo_root}/features/items/edit/run-edit-item-tests.log
#
# The artifacts produced are:
#   - ${repo_root}/features/artifacts/reports/*.html
#

## Exit on any non-zero exit status
#
#
# set -euxo pipefail

## Run Robot test files
#
pushd ~/src/dynahist/src
  rm -f run-edit-item-tests.log
  while IFS= read -r -d '' file; do
    ## Strip folder prefix
    #
    # file="${file##*/}"
    # folder=$file
    ## Strip file extension
    #
    ID="${file%.java}"
    echo "Running ${file}"
    java -jar ~/src/java-to-rust/java-2-rust.jar ${file}
    # java -jar java-2-rust.jar test/
    # robot -o ./../artifacts/reports/${ID}.xml \
    #       -l ./../artifacts/reports/${ID}.log \
    #       -r ./../artifacts/reports/${ID}.html \
    #       -x ./../artifacts/reports/${ID}.xunit.xml \
    #       ${ID}.robot & >> run-edit-item-tests.log
  done < <(find . -name "*.java" \
                  -print0 | \
                  sort -z
          )
popd
