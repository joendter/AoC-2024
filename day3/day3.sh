c_brackets="))))))))))"
o_brackets="(((((((((("
o100="$o_brackets$o_brackets$o_brackets$o_brackets$o_brackets$o_brackets$o_brackets$o_brackets$o_brackets$o_brackets"
stuff=$(
    cat $1 |
    sed -E "s/\)/)\n/g" | # split everything to separate lines for grep to work properly
    grep -o -E "(mul\([0-9]{1,3},[0-9]{1,3}\))|(do(n't)?\(\))" | 
    sed -E "s/mul\(([0-9]*),([0-9]*)\)/\1*\2/" | 
    tr '\n' ' ' |
    sed -E "s/do\(\)/$c_brackets/g" | 
    sed -E "s/don't\(\)/0*(/g" |
    sed -E "s/(.*)/print$o100$o100$o100$o_brackets$o_brackets$o_brackets$o_brackets$o_brackets$o_brackets(((((\1$c_brackets$c_brackets$c_brackets$c_brackets$c_brackets$c_brackets/" |
    sed -E "s/([0-9)]) ([0-9])/\1+\2/g"
)

function my_test {
    echo "$1" |
        python3
}

echo "$stuff"
echo "$stuff" |
    sed -E "s/\(/(\n/g" |
    wc -l
echo "$stuff" |
    sed -E "s/\)/)\n/g" |
    wc -l
echo "$stuff" | 
    python3
