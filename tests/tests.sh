cat ftest | while read p; do cargo test ${p}; sleep 2; done
