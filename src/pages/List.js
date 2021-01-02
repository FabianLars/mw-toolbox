import { Flex } from "@chakra-ui/react";
import React from "react";
import Header from "../components/sections/Header";

const List = () => {
    return (
        <Flex direction="column" align="center" maxW={{ xl: '1240px' }} m="0 auto" h="100vh">
            <Header />

        </Flex>
    );
}

export default List;