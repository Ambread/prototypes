from functools import reduce
import re


def my_max(x, y):
    """Exercise 1"""
    if x > y:
        return x
    return y


def max_of_three(x, y, z):
    """Exercise 2"""
    if x > y and x > z:
        return x
    if y > x and y > z:
        return y
    return z


def my_len(input):
    """Exercise 3"""
    i = 0
    for _ in input:
        i += 1
    return i


def is_vowel(char):
    """Exercise 4"""
    return char in ["a", "e", "i", "o", "u"]


def robber_language(input):
    """Exercise 5"""
    output = ""
    for char in input:
        if is_vowel(char) or char == " ":
            output += char
        else:
            output += char + "o" + char
    return output


def sum(input):
    """Exercise 6"""
    output = 0
    for num in input:
        output += num
    return output


def multiply(input):
    """Exercise 6"""
    output = 1
    for num in input:
        output *= num
    return output


def reverse(input):
    """Exercise 7"""
    output = ""
    for char in input:
        output = char + output
    return output


def is_palindrome(input):
    """Exercise 8"""
    return input == reverse(input)


def is_member(needle, haystack):
    """Exercise 9"""
    for item in haystack:
        if item == needle:
            return True
    return False


def overlapping(inputX, inputY):
    """Exercise 10"""
    for x in inputX:
        for y in inputY:
            if x == y:
                return True
    return False


def generate_n_chars(n, char):
    """Exercise 11"""
    output = ""
    for _ in range(n):
        output += char
    return output


def histogram(input):
    """Exercise 12"""
    output = ""
    for n in input:
        output += "*" * n + "\n"
    return output


def max_in_list(input):
    """Exercise 13"""
    output = 0
    for x in input:
        if x > output:
            output = x
    return output


def word_lengths(input):
    """Exercise 14"""
    output = []
    for word in input:
        output.append(len(word))
    return output


def find_longest_word(input):
    """Exercise 15"""
    output = ""
    for word in input:
        if len(word) > len(output):
            output = word
    return output


def filter_long_words(n, input):
    """Exercise 16"""
    output = []
    for word in input:
        if len(word) > n:
            output.append(word)
    return output


def is_palandrome_phrase(input):
    """Exercise 17"""
    filtered = ""
    for char in input:
        if char.isalpha():
            filtered += char.lower()
    return filtered == reverse(filtered)


def is_pangram(input):
    """Exercise 18"""
    input = input.lower()
    for char in "abcdefghijklmnopqrstuvwxyz":
        if char not in input:
            return False
    return True


def translate_card(input):
    """Exercise 19"""
    lexicon = {"merry": "god", "christmas": "jul", "and": "och", "happy": "gott", "new": "nytt",
               "year": "år"}
    output = []
    for word in input:
        output.append(lexicon[word])
    return output


def char_freq(input):
    """Exercise 20"""
    chars = {}
    for char in input:
        if char in chars:
            chars[char] += 1
        else:
            chars[char] = 1
    return chars


def rot13(input):
    """Exercise 21"""
    key = {'a': 'n', 'b': 'o', 'c': 'p', 'd': 'q', 'e': 'r', 'f': 's', 'g': 't', 'h': 'u',
           'i': 'v', 'j': 'w', 'k': 'x', 'l': 'y', 'm': 'z', 'n': 'a', 'o': 'b', 'p': 'c',
           'q': 'd', 'r': 'e', 's': 'f', 't': 'g', 'u': 'h', 'v': 'i', 'w': 'j', 'x': 'k',
           'y': 'l', 'z': 'm', 'A': 'N', 'B': 'O', 'C': 'P', 'D': 'Q', 'E': 'R', 'F': 'S',
           'G': 'T', 'H': 'U', 'I': 'V', 'J': 'W', 'K': 'X', 'L': 'Y', 'M': 'Z', 'N': 'A',
           'O': 'B', 'P': 'C', 'Q': 'D', 'R': 'E', 'S': 'F', 'T': 'G', 'U': 'H', 'V': 'I',
           'W': 'J', 'X': 'K', 'Y': 'L', 'Z': 'M'}
    output = ""
    for char in input:
        if char.isalpha():
            output += key[char]
        else:
            output += char
    return output


def correct(input):
    """Exercise 22"""
    input = re.sub("\s+", " ", input)
    input = re.sub("\.(?=\w)", ". ", input)
    return input


def make_3sg_form(input):
    """Exercise 23"""
    if input.endswith("y"):
        return input[:-1] + "ies"
    for case in ["o", "ch", "s", "sh", "x", "z"]:
        if input.endswith(case):
            return input + "es"
    return input + "s"


def make_ing_form(input):
    """Exercise 24"""
    if input.endswith("ie"):
        return input[:-2] + "ying"
    if input.endswith("e"):
        return input[:-1] + "ing"
    if (
        len(input) == 3
        and not is_vowel(input[0])
        and is_vowel(input[1])
        and not is_vowel(input[2])
    ):
        return input + input[2] + "ing"
    return input + "ing"


def max_in_list_func(input):
    """Exercise 25"""
    return reduce(max, input, 0)


def word_lengths_func(input):
    """Exercise 26"""
    return list(map(len, input))


def word_lengths_comprehention(input):
    """Exercise 26"""
    return [len(word) for word in input]


def find_longest_word_func(input):
    """Exercise 27"""
    return reduce(max, map(len, input), 0)


def filter_long_words_func(n, input):
    """Exercise 28"""
    return list(filter(lambda word: len(word) > n, input))


def translate_card_func(input):
    """Exercise 29"""
    lexicon = {"merry": "god", "christmas": "jul", "and": "och", "happy": "gott", "new": "nytt",
               "year": "år"}
    return list(map(lambda word: lexicon[word], input))


def my_map(func, input):
    """Exercise 30"""
    output = []
    for item in input:
        output.append(func(item))
    return output


def my_filter(func, input):
    """Exercise 30"""
    output = []
    for item in input:
        if func(item):
            output.append(item)
    return output


def my_reduce(func, input, initial):
    """Exercise 30"""
    output = initial
    for item in input:
        output = func(item, output)
    return output
