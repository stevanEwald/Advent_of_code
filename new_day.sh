cp -r aoc_template $1
sed -i.bak -E "s/^name = aoc/name =\"$1\"/" ./$1/Cargo.toml

FILE="Cargo.toml"
for member in "$@"; do
    # Check if the member already exists in the list
    if grep -q "\"$member\"" "$FILE"; then
        echo "$member is already in the list."
    else
        # Add the new member before the closing bracket
        sed -i.bak "/members = \[/a \    \"$member\"," "$FILE"
        echo "$member added to the list."
    fi
done