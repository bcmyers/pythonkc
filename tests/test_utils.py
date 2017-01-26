import unittest

from pythonkc.utils import parse_args, DEFAULT_BOUND, DEFAULT_NPROCS


class TestUtils(unittest.TestCase):

    def test_parse_args_1(self):
        args = parse_args([])
        self.assertEqual(args.bound, DEFAULT_BOUND)
        self.assertEqual(args.nprocs, DEFAULT_NPROCS)

    def test_parse_args_2(self):
        args = parse_args(['5'])
        self.assertEqual(args.bound, 5)
        self.assertEqual(args.nprocs, DEFAULT_NPROCS)

    def test_parse_args_3(self):
        args = parse_args(['--nprocs', '4'])
        self.assertEqual(args.bound, DEFAULT_BOUND)
        self.assertEqual(args.nprocs, 4)

    def test_parse_args_4(self):
        args = parse_args(['5', '--nprocs', '4'])
        self.assertEqual(args.bound, 5)
        self.assertEqual(args.nprocs, 4)


if __name__ == '__main__':
    unittest.main()
