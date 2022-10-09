import * as React from "react";
import image from "./../images/dokc-logo-white.svg";

let Guide = () => {
  return (
    <>
      <section className="pt-8 grid lg:grid-cols-10 sm:grid-cols-1 gap-6">
        <section className="sm:col-start-1 sm:justify-center lg:col-start-3 col-span-2">
          <section class="py-8 px-3">
            <img
              className="inline"
              src={image}
              alt="Data on Kubernetes community logo"
            />
          </section>
        </section>
        <section className="sm:col-start-1 lg:text-right lg:col-start-5 lg:col-span-4">
          <section className="px-3 text-white lg:text-4xl md:text-2xl sm:text-lg font-semibold capitalize hover:text-white">
            Welcome to the Data on Kubernetes Community!
            <section className="md:h-[0.5px] lg:h-[2px] w-auto bg-white  my-2" />
            <section className="text-white font-light normal-case lg:text-2xl md:text-xl sm:text-md hover:text-white">
              A place for learning and running databases and other stateful
              services on Kubernetes.
              <br />
            </section>
          </section>
        </section>
      </section>
      <section className="pt-8 grid lg:grid-cols-12">
        <section className="col-start-3 col-span-8">
          <section className="text-white font-bold text-4xl items-center font-mono">
            What is Open Source ?
          </section>
          <section className="pt-6 text-white font-regular text-2xl font-mono">
            #How to get started? #What does it mean ? #Why should you ?
          </section>
          <section className="py-2 text-white font-regular text-lg">
            Generally, Open Source software is software that can be freely
            accessed, used, changed, and shared (in unabridged form) by anyone.
            Open source software is made by many people, and distributed under
            licenses that comply with the Open Source Definition. Some famous
            open source licenses are MIT, BSD-II, BSD-III.
          </section>
          <section className="py-2 text-3xl text-white font-mono font-semibold">
            How should you get started ?
          </section>
          <section className="py-2 text-white font-regular text-lg">
            To get started with open source, go to your favourite repository,
            For example{" "}
            <a
              href="https://github.com/dokc/dokc.github.io"
              className="underline"
            >
              this github page .
            </a>
            To contribute to this project, you will have to fork the repository,
            you can fork it by clicking on the fork button. Once forked you
            should be able to see the repository in your account. The repository
            will be available to you and you can maintain the fork if you choose
            to build a new stream of features. If you want to implement a
            feature on the parent repository, clone it.
            <section className="p-2 text-white font-light font-mono text-md flex justify-space-around px-8">
              git clone [your repository url]
            </section>
            Once cloned, you can go on to repository and implement the necessary
            changes.
            <b className="px-2">
              {" "}
              Before you commit the changes, ensure you've read the contributing
              guide to avoid potential rejections.
            </b>
            Once completed, you can jump into the pull request section of this
            blog.
          </section>
          <section className="text-3xl text-white font-mono font-semibold">
            How to make a pull request / merge request ?
          </section>
          <section className="py-2 text-white font-regular text-lg">
            <b>Congratulations</b>, you've made your first changes, locally. Now
            these changes need to be pushed into your remote repository hosted
            on <i className="emphasis"> GitHub / GitLab</i>. To do so you can
            run the following commands.
            <section className="p-2 text-white font-light font-mono text-md flex justify-space-around px-8">
              git commit -s -m [Your Message]
            </section>
            Once you've commited, you can push it to your remote using this
            command.
            <section className="p-2 text-white font-light font-mono text-md flex justify-space-around px-8">
              git push -u [remote-name] [branch-name]
            </section>
            You've pushed the changes to repository, now you will be prompted by
            GitHub to create a pull request.
            <br/>
            <em className="py-6 text-3xl font-mono">
              VOILA ! You have made your first ever contribution to an
              open-source project
            </em>
          </section>
        </section>
      </section>
    </>
  );
};

export default Guide;
