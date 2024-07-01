import { useState } from 'react';
import Face from './Face';

export default function CubeDisplay({ faces }) {
    if (faces.length) {
        return (
            <>
                <div className="grid grid-cols-4 w-full gap-4">
                    <div></div>
                    <div>
                        <Face key="Up" data={faces[0]} />
                    </div>
                    <div></div>
                    <div></div>
                    <div>
                        <Face key="Left" data={faces[1]} />
                    </div>
                    <div>
                        <Face key="Front" data={faces[2]} />
                    </div>
                    <div>
                        <Face key="Right" data={faces[3]} />
                    </div>
                    <div>
                        <Face key="Back" data={faces[4]} />
                    </div>
                    <div></div>
                    <div>
                        <Face key="Down" data={faces[5]} />
                    </div>
                    <div></div>
                    <div></div>
                </div>
            </>
        );
    }
    return <></>;
}
