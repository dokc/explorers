import { Link } from "gatsby";
import * as React from "react";
import { useState } from "react";
import { Menu } from "@headlessui/react";
import {
  MenuItemProps,
  GenericProps as DropDownMenuProps,
} from "../../types/props";

import { HiOutlineMenuAlt2 } from "react-icons/hi";
import { AiFillCloseCircle } from "react-icons/ai";

let MenuItem = ({ icon, text, link }: MenuItemProps): JSX.Element => {
  return (
    <>
      <Menu.Item>
        <Link to={link}>
          <button className="z-2 top flex justify-start gap-2 bg-white text-black font-medium hover:bg-emerald-300 hover:font-bold group w-full items-center rounded-md p-2 text-md">
            <section className="m-1">{icon}</section>
            {text}
          </button>
        </Link>
      </Menu.Item>
    </>
  );
};

let DropDownMenu = (props: DropDownMenuProps): JSX.Element => {
  const [isOpen, setIsOpen] = useState(true);

  const OpenJSX = (): JSX.Element => {
    if (isOpen) {
      return (
        <section className="">
          <button onClick={() => setIsOpen(!isOpen)}>
            <HiOutlineMenuAlt2 className="text-white" aria-hidden="true" />
          </button>
        </section>
      );
    } else {
      return (
        <section>
          <button onClick={() => setIsOpen(!isOpen)}>
            <AiFillCloseCircle className="text-white" aria-hidden="true" />
          </button>
        </section>
      );
    }
  };

  return (
    <section className="w-56 text-left">
      <Menu as="section" className="relative inline-block text-left">
        <section>
          <Menu.Button className="flex justify-center rounded-md hover:bg-gray-700 p-2 bg-black text-white text-2xl ">
            {OpenJSX()}
          </Menu.Button>
        </section>
        <Menu.Items className="absolute left mt-2 rounded-md bg-white shadow-lg w-56">
          <section className="p-1">{props.children}</section>
        </Menu.Items>
      </Menu>
    </section>
  );
};

export { DropDownMenu, MenuItem };
