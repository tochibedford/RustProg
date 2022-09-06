import os
import sys

def main():
    if os.path.exists(os.path.join(os.getcwd(), sys.argv[1])):
        raise OSError('Path Already Exists')

    print('Bootstrapping a rust project in: ' + os.path.join(os.getcwd(),sys.argv[1]))

    os.mkdir(os.path.join(os.getcwd(), sys.argv[1]))
    fileName = "main.rs"
    with open(os.path.join(os.getcwd(), sys.argv[1], fileName), 'x') as rustMainFile:
        rustMainFile.write("""fn main(){
    println!("hello world!");
}
""")

if __name__ == '__main__':
    main()