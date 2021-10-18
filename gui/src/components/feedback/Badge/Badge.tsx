import cls from './Badge.module.css';

type Props = {
    children: React.ReactNode;
    type?: 'success' | 'error';
};

const Badge = ({ children, type }: Props): JSX.Element => {
    return <span className={`${cls.badge} ${type ? cls[type] : ''}`}>{children}</span>;
};

export default Badge;
