from operator import add
import steve
import unittest


class Test(unittest.TestCase):
    def test_my_max(self):
        self.assertEqual(steve.my_max(9, 23), 23)
        self.assertEqual(steve.my_max(43, 17), 43)

    def test_max_of_three(self):
        self.assertEqual(steve.max_of_three(9, 23, 1), 23)
        self.assertEqual(steve.max_of_three(43, 17, 5), 43)
        self.assertEqual(steve.max_of_three(31, 25, 76), 76)

    def test_my_len(self):
        self.assertEqual(steve.my_len([1, 2, 3, 4]), 4)
        self.assertEqual(steve.my_len("wew"), 3)

    def test_is_vowel(self):
        self.assertEqual(steve.is_vowel("t"), False)
        self.assertEqual(steve.is_vowel("a"), True)

    def test_robber_language(self):
        self.assertEqual(steve.robber_language(
            "this is fun"), "tothohisos isos fofunon")

    def test_sum(self):
        self.assertEqual(steve.sum([1, 2, 3, 4]), 10)

    def test_multiply(self):
        self.assertEqual(steve.multiply([1, 2, 3, 4]), 24)

    def test_reverse(self):
        self.assertEqual(steve.reverse("foobar"), "raboof")
        self.assertEqual(steve.reverse("tacocat"), "tacocat")

    def test_is_palindrome(self):
        self.assertEqual(steve.is_palindrome("foobar"), False)
        self.assertEqual(steve.is_palindrome("tacocat"), True)

    def test_is_member(self):
        self.assertEqual(steve.is_member(5, [1, 2, 3, 4]), False)
        self.assertEqual(steve.is_member(6, [5, 6, 7, 8]), True)

    def test_overlapping(self):
        self.assertEqual(steve.overlapping([1, 2, 3, 4], [5, 6, 7, 8]), False)
        self.assertEqual(steve.overlapping([1, 2, 3], [3, 4, 5]), True)

    def test_generate_n_chars(self):
        self.assertEqual(steve.generate_n_chars(5, "x"), "xxxxx")

    def test_histogram(self):
        self.assertEqual(steve.histogram(
            [4, 9, 7]), "****\n*********\n*******\n")

    def test_max_in_list(self):
        self.assertEqual(steve.max_in_list([4, 85, 56, 73]), 85)

    def test_word_lengths(self):
        self.assertEqual(steve.word_lengths(
            ["wew", "foobar", "aaaa"]), [3, 6, 4])

    def test_find_longest_word(self):
        self.assertEqual(steve.find_longest_word(
            ["wew", "foobar", "aaaa"]), "foobar")

    def test_filter_long_words(self):
        self.assertEqual(steve.filter_long_words(
            3, ["wew", "foobar", "aaaa"]), ["foobar", "aaaa"])

    def test_is_palandrome_phrase(self):
        self.assertEqual(steve.is_palandrome_phrase("w, wewhu, r'w"), False)
        self.assertEqual(steve.is_palandrome_phrase(
            "Go hang a salami I'm a lasagna hog."), True)

    def test_is_pangram(self):
        self.assertEqual(steve.is_pangram(
            "The quick brown fox jumps over the lazy dog."), True)
        self.assertEqual(steve.is_pangram(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaa"), False)

    def test_translate_card(self):
        self.assertEqual(steve.translate_card(
            ["merry", "christmas", "and", "happy", "new", "year"]), ['god', 'jul', 'och', 'gott', 'nytt', 'år'])

    def test_char_freq(self):
        self.assertEqual(steve.char_freq("wew"), {"w": 2, "e": 1})

    def test_rot13(self):
        self.assertEqual(steve.rot13(
            "Pnrfne pvcure? V zhpu cersre Pnrfne fnynq!"), "Caesar cipher? I much prefer Caesar salad!")

    def test_correct(self):
        self.assertEqual(steve.correct(
            "This is  very funny  and    cool.Indeed!"),  "This is very funny and cool. Indeed!"
        )

    def test_make_3sg_form(self):
        self.assertEqual(steve.make_3sg_form("try"), "tries")
        self.assertEqual(steve.make_3sg_form("brush"), "brushes")
        self.assertEqual(steve.make_3sg_form("run"), "runs")

    def test_make_ing_form(self):
        self.assertEqual(steve.make_ing_form("lie"), "lying")
        self.assertEqual(steve.make_ing_form("move"), "moving")
        self.assertEqual(steve.make_ing_form("hug"), "hugging")

    def test_max_in_list_func(self):
        self.assertEqual(steve.max_in_list_func([4, 85, 56, 73]), 85)

    def test_word_lengths_func(self):
        self.assertEqual(steve.word_lengths_func(
            ["wew", "foobar", "aaaa"]), [3, 6, 4])

    def test_word_lengths_comprehention(self):
        self.assertEqual(steve.word_lengths_comprehention(
            ["wew", "foobar", "aaaa"]), [3, 6, 4])

    def test_filter_long_words_func(self):
        self.assertEqual(steve.filter_long_words_func(
            3, ["wew", "foobar", "aaaa"]), ["foobar", "aaaa"])

    def test_translate_card_func(self):
        self.assertEqual(steve.translate_card_func(
            ["merry", "christmas", "and", "happy", "new", "year"]), ['god', 'jul', 'och', 'gott', 'nytt', 'år'])

    def test_my_map(self):
        self.assertEqual(steve.my_map(len, ["wew", "aaaa"]), [3, 4])

    def test_my_filter(self):
        self.assertEqual(steve.my_filter(len, ["wew", ""]), ["wew"])

    def test_my_reduce(self):
        self.assertEqual(steve.my_reduce(add, [1, 2, 3, 4], 1), 11)


if __name__ == "__main__":
    unittest.main()
