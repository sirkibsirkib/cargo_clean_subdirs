import sys, os, glob, subprocess
from multiprocessing import Pool


def work(abs_path):
	os.chdir(abs_path)
	print(abs_path)
	subprocess.call(["cargo", "clean"])

def main():
	root = sys.argv[1]
	print("will run `cargo clean` in all dirs in `{}`".format(root))
	input("Press Enter to continue (or interrupt to abort) ")
	abs_dirs = [os.path.abspath(d) for d in os.listdir(root)]
	pool = Pool()
	pool.map(work, abs_dirs)

if __name__ == '__main__':
	main()