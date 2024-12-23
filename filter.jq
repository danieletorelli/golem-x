def sort_keys:
  . as $in
  | if type == "object" then
      to_entries
      | map(.value |= sort_keys)  # Recursively sort nested objects
      | sort_by(.key)
      | from_entries
    elif type == "array" then
      if all(.[]; type == "object") then
        map(sort_keys) | sort_by(.author, .content)  # Sort arrays of objects by specific keys
      elif all(.[]; type == "object" or type == "array") then
        map(sort_keys)  # Recursively process arrays with nested objects/arrays
      else
        sort  # Sort arrays of simple values (e.g., strings, numbers)
      end
    else
      .
    end;

def remove_timestamps:
  if .tweets then
    .tweets |= map(del(.timestamp))
  elif .tweet then
    .tweet |= del(.timestamp)
  else
    .
  end;

sort_keys | remove_timestamps