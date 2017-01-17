import codecs
import os
from pip.req import parse_requirements
from setuptools import find_packages, setup

BASE_DIR = os.path.abspath(os.path.dirname(os.path.dirname(__file__)))

REQUIREMENTS = parse_requirements(
    os.path.join(BASE_DIR, 'python', 'requirements', 'production.txt'),
    session=False)
REQUIREMENTS = [str(r.req) for r in REQUIREMENTS]

with codecs.open(os.path.join(BASE_DIR, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()

setup(
    author='Brian Myers',
    author_email='brian.carl.myers@gmail.com',
    classifiers=[
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: MIT License',
        'Natural Language :: English',
        'Operating System :: MacOS :: MacOS X',
        'Operating System :: Microsoft :: Windows',
        'Operating System :: POSIX :: Linux',
        'Programming Language :: Python :: 3.6'],
    description='Template for calling Rust from Python',
    install_requires=REQUIREMENTS,
    keywords='rust',
    license='MIT',
    long_description=long_description,
    name='pythonkc',
    packages=find_packages(exclude=['docs']),
    url='https://www.github.com/bcmyers/pythonkc',
    version='0.1.0')
