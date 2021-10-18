import cls from './FileList.module.css';

const FileList = ({
    children,
    placeholder,
}: {
    children: JSX.Element[];
    placeholder?: string;
}): JSX.Element => {
    return (
        <div className={`${cls.flist} ${children.length === 0 ? cls.empty : ''}`}>
            {children.length === 0 ? placeholder : children}
        </div>
    );
};

export default FileList;
