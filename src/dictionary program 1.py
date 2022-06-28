import json
from difflib import get_close_matches as gcm
# from typing import ClassVar, get_type_hints


data = json.load(open("../temp_files/data.txt"))
# print(data)
while True:
    def lookup(word):
        result = gcm(word, data.keys())
        if word.lower() in data:
            print("")
            data_list = data[word.lower()]
            for item in data_list:
                return f"Definition: {item}"
        elif word.title() in data:
            print("")
            data_list = data[word.title()]
            for item in data_list:
                return f"Definition: {item}"
        elif word.upper() in data:
            print("")
            data_list = data[word.upper()]
            for item in data_list:
                return f"Definition: {item}"
        elif len(result) > 0:
            yn = input(
                'Did you mean %s instead? Enter Y if yes, N if no.' % result[0])
            if yn.upper() == "Y":
                for item in data[result[0]]:
                    return f"\nDefinition: {item}"
        elif len(result) > 0:
            return "The word doesn't exist, try again."
        else:
            if len(result) == 0:
                return "The word doesn't exist, try again."
        return
    search_word = input(
        "\nEnter word to you would like to look up, or CTRL-C to exit: ")
    print(lookup(search_word))
