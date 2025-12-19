import { useState } from "react"
import { simulate } from "../../lilypond-simulation/pkg/lilypond_simulation"
import { Stage, Layer, Circle } from 'react-konva';

function App() {
  const [dimension, setDimension] = useState(2);
  const [numPoints, setNumPoints] = useState(5);
  const [result, setResult] = useState<any>(null);

  const handleSimulate = () => {
    const simulationResult = simulate(dimension, numPoints);
    setResult(simulationResult);
  };
  
  let width = 200;

  return (
    <>
      <div>
        <h2>Lilypond Simulation</h2> </div>

      <div>
        <h3>Parameters</h3>
        <div>
          <div>
            <label htmlFor="dimension">Dimension: </label>
            <input
              id="dimension"
              type="number"
              min="1"
              value={dimension}
              onChange={(e) => setDimension(parseInt(e.target.value) || 1)}
            />
          </div>
          <div>
            <label htmlFor="numPoints">Number of Points: </label>
            <input
              id="numPoints"
              type="number"
              min="1"
              value={numPoints}
              onChange={(e) => setNumPoints(parseInt(e.target.value) || 1)}
            />
          </div>
        </div>
        <button onClick={handleSimulate}>
          Simulate
        </button>
      </div>

      {result && (
        <div>
          <h3>Simulation Result</h3>
          <div
            style={{
              display: "inline-block",
              border: "2px solid #000",
              padding: "0",
            }}
          >
            <Stage width={width} height={width}>
              <Layer>
                {result.map((circle: any, i: number) => (
                  <Circle 
                    key={i}
                    x={width * circle.point[0]} 
                    y={width * circle.point[1]} 
                    radius={width * circle.radius}
                    fill="green"
                  />
                ))}
              </Layer>
            </Stage>
          </div>
          <pre>
            {JSON.stringify(result, null, 2)}
          </pre>
        </div>
      )}
    </>
  )
}

export default App
