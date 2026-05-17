for file in ./tests/*; do
  cargo run -- "$file"
  nasm "${file}.asm" -o "${file}_reassembled"
  diff_output=$(diff "$file" "${file}_reassembled")
  if [ -n "$diff_output" ]; then
    echo "FAILED: $file"
    echo "$diff_output"
  fi
  rm "${file}.asm" "${file}_reassembled"
done
