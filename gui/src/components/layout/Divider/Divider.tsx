import cls from './Divider.module.css';

type Props = {
    orientation?: 'horizontal' | 'vertical';
};

const Divider = ({ orientation = 'horizontal' }: Props) => {
    return (
        <hr
            aria-orientation={orientation as Props['orientation']}
            className={`${cls.hr} ${cls[orientation]}`}
        />
    );
};

export default Divider;
