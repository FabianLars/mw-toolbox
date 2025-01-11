import cls from './Spinner.module.css';

type Props = { className?: string };

const Spinner = ({ className = '' }: Props) => {
    return (
        <div className={`${cls.spinner} ${className}`}>
            {/* <span className={cls.span}>Loading...</span> */}
        </div>
    );
};

export default Spinner;
