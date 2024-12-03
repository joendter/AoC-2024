stuff=$(
    cat $1 |
    sed -E "s/\)/)\n/g" | # split everything to separate lines for grep to work properly
    grep -o -E "(mul\([0-9]{1,3},[0-9]{1,3}\))|(do(n't)?\(\))" | 
    sed -E "s/mul\(([0-9]*),([0-9]*)\)/\1*\2/" | 
    tr '\n' ' ' |
    sed -E "s/do\(\)/ö/g" |
    sed -E "s/don't\(\)[^ö]*?ö//g" |
    sed -E "s/ö//g" |
    sed -E "s/don't\(\).*$//g" |
    sed -E "s/(.*)/print(\1)/" |
    sed -E "s/([0-9)]) +([0-9])/\1+\2/g"
)

echo "$stuff"
echo "$stuff" | 
    python3
