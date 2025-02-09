import { component$, useStore, $, useSignal, useTask$ } from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";

interface ITask {
  id: number;
  name: string;
  done: boolean;
}

type FilterStateType = "all" | "active" | "completed";

export default component$(() => {
  const store = useStore({
    task: "",
    tasks: [
      {
        id: 1,
        name: "Buy groceries",
        done: false,
      },
      {
        id: 2,
        name: "Walk the dog",
        done: true,
      },
    ],
  });

  const displayedTasks = useSignal<ITask[] | undefined>();
  const filterSignal = useSignal<FilterStateType>("all");

  useTask$(({ track }) => {
    // track(() => store.filter);
    // track(() => store.tasks);
    track(() => [filterSignal, store.tasks]);

    console.log(filterSignal.value);

    switch (filterSignal.value) {
      case "all":
        displayedTasks.value = store.tasks;
        break;
      case "active":
        displayedTasks.value = store.tasks.filter(
          (task) => task.done === false,
        );
        break;
      case "completed":
        displayedTasks.value = store.tasks.filter((task) => task.done === true);
        break;
    }
  });

  const addTask = $((evt: SubmitEvent) => {
    evt.preventDefault();

    store.tasks = [
      ...store.tasks,
      {
        id: store.tasks.length + 1,
        name: store.task,
        done: false,
      },
    ];

    store.task = "";
  });

  const toggleTaskCompletion = $((id: number) => {
    store.tasks = store.tasks.map((task) =>
      task.id === id ? { ...task, done: !task.done } : task,
    );
  });

  return (
    <main class="container mx-auto bg-slate-100 p-4 lg:w-[50%]">
      <h1 class="place-self-center text-4xl text-slate-700">Qwik Tasks</h1>

      <ul class="mt-4">
        {displayedTasks.value?.map((task: ITask) => (
          <li key={task.id} class="flex items-center">
            <input
              type="checkbox"
              checked={task.done}
              class="mr-2"
              onClick$={() => toggleTaskCompletion(task.id)}
            />
            <span
              class={{
                "line-through": task.done,
              }}
            >
              {task.name}
            </span>
          </li>
        ))}
      </ul>

      <form class="mt-4 flex" onSubmit$={addTask} preventdefault:submit>
        <input
          type="text"
          placeholder="Add a new task..."
          name="task"
          class="w-full flex-1 rounded border border-slate-300 p-2 outline-0"
          value={store.task}
          onInput$={(evt) =>
            (store.task = (evt.target as HTMLInputElement).value)
          }
        />

        <button type="submit" class="rounded bg-slate-800 p-4 text-white">
          Add
        </button>
      </form>

      <select name="" id="">
        {["all", "active", "completed"].map((filter) => (
          <option
            key={filter}
            value={filter}
            onSelect$={() => (filterSignal.value = filter as FilterStateType)}
          >
            {filter}
          </option>
        ))}
      </select>
    </main>
  );
});

export const head: DocumentHead = {
  title: "Todo List",
  meta: [
    {
      name: "Qwik Tasks",
      content: "A simple task manager web app using Qwik.",
    },
  ],
};
