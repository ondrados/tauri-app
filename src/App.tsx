import { invoke } from '@tauri-apps/api/tauri';
import { Component, createSignal } from 'solid-js';
import './App.scss';
import Counter from './Counter';

const App: Component = () => {
  const [counter, setCounter] = createSignal(0);
  setInterval(setCounter, 1000, (c: number) => c + 1);

  async function handleClick(event){

    // Invoke the command from our backend using the invoke function
    const result = await invoke("return_string", {
      word: "This is the argument"
    })

    // print the result to an alert
    alert(result)
  }

  async function setKey(event){

    // Invoke the command from our backend using the invoke function
    const result = await invoke("set_key", {
      key: "my-key",
      value: 99
    })

  }

  async function getKey(event){

    // Invoke the command from our backend using the invoke function
    const result = await invoke("get_key", {
      key: "my-key"
    })

    // print the result to an alert
    alert(result)
  }

  return (
    <>
      <div>
        <h1 class="header">{counter()}</h1>
        <button onClick={handleClick}>Click Me</button>
        <button onClick={setKey}>Set key</button>
        <button onClick={getKey}>Get key</button>
      </div>
      <Counter />
    </>
  );
};

export default App;
