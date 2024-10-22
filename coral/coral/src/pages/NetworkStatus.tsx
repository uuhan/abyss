import React from 'react';

export default function NavigatorOnline(props: { onChange: (onLine:Boolean) => void }) {
    // if when navigator.Online change, we can get value. (true or false)
    const getStatus = () => {
        if (props.onChange) {
            props.onChange(navigator.onLine)
        }
    }

    // windows event listener (online and offline)
    React.useEffect(() => {
        window.addEventListener('online', getStatus);
        window.addEventListener('offline', getStatus);
    })

    return null;
}
