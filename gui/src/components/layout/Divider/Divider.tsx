import cls from './Divider.module.css';

type Props = {
    orientation?: 'horizontal' | 'vertical';
};

const Divider = ({ orientation = 'horizontal' }: Props): JSX.Element => {
    return (
        <hr
            aria-orientation={orientation as Props['orientation']}
            className={`${cls.hr} ${cls[orientation]}`}
        />
    );
};

export default Divider;
